use std::env;
use std::fs;
use std::collections::HashMap;

use regex::Regex;

use ethabi::{Contract, Token, Param};
use ethabi::ethereum_types::{U256};
use serde_json;
use serde_json::Value;

use web3::futures::Future;
use web3::types::{BlockNumber, FilterBuilder};
use web3::types::{Transaction, TransactionReceipt, TransactionId};
use web3::types::{Address, Bytes, H256, H160};

const V1_UNISWAP_FACTORY: &str = "0xc0a47dFe034B400B47bDaD5FecDa2621de6c4d95";

#[tokio::main]
async fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
			eprintln!("Error: missing tx hash argument");
			std::process::exit(1);
	}

	let hash = &args[1];
	let re = Regex::new(r"^0x[0-9a-fA-F]{64}$").unwrap();
	if !re.is_match(hash) {
			println!("Error: malformed tx hash argument");
			std::process::exit(1);
	}

	let config_str = fs::read_to_string("./config/config.json").expect("Error: Failed to read config file");
	let config: Value = serde_json::from_str(&config_str).expect("Error: Failed to parse JSON");
	let rpc_provider_url = config["rpc-provider-url"].as_str().expect("Error: Failed to get rpc-provider-url");

	let transport = web3::transports::Http::new(rpc_provider_url).unwrap();
	let web3 = web3::Web3::new(transport);
	let tx_id = TransactionId::Hash(hash.parse().unwrap());
	let tx_result = web3.eth().transaction(tx_id).await;

	let successful_tx: Transaction;
	let successful_receipt: TransactionReceipt;
	match tx_result {
		Ok(Some(tx)) => {
			println!("Caller: {:?}", tx.from);
			if tx.block_number.is_none() {
					println!("Error: tx is still pending");
					std::process::exit(1);
			} else {
					// To check if a transaction was reverted or successful, you need to check the receipt of the transaction
					let h256_hash = hash.parse::<H256>().unwrap();
					let receipt = web3.eth().transaction_receipt(h256_hash).await.unwrap();
					match receipt {
							Some(receipt) => {
									if receipt.status == Some(web3::types::U64([1])) {
											successful_tx = tx;
											successful_receipt = receipt;
									} else {
											println!("Error: tx was reverted");
											std::process::exit(1);
									}
							}
							None => {
								println!("Error: tx doesn't exist");
								std::process::exit(1);
							},
					}
			}
		}
		Ok(None) => {
			println!("Error: tx doesn't exist");
			std::process::exit(1);
		},
		Err(e) => {
			println!("Error: {}", e);
			std::process::exit(1);
		},
	}

	let mut abi_paths: HashMap<&str, &str> = HashMap::new();
	abi_paths.insert("uniswap_v1", "./abi/eth/uniswap_v1/exchange.json");

	let mut tokens: HashMap<&str, &str> = HashMap::new();
	tokens.insert("usdt", "0xdAC17F958D2ee523a2206206994597C13D831ec7");
	tokens.insert("usdc", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
	tokens.insert("busd", "0x4Fabb145d64652a948d72533023f6E7A623C7C53");
	tokens.insert("bnb", "0xB8c77482e45F1F44dE1745F52C74426C631bDD52");
	tokens.insert("matic", "0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0");
	tokens.insert("shib", "0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE");

	let mut uniswap_v1_dexes: HashMap<&str, &str> = HashMap::new();
	uniswap_v1_dexes.insert("usdt", "0xc8313c965C47D1E0B5cDCD757B210356AD0e400C");
	uniswap_v1_dexes.insert("usdc", "0x97deC872013f6B5fB443861090ad931542878126");
	uniswap_v1_dexes.insert("busd", "0x25C610eeE8f59768c26567c388986Aab3467a3E3");
	uniswap_v1_dexes.insert("bnb", "0x255e60c9d597dCAA66006A904eD36424F7B26286");
	uniswap_v1_dexes.insert("matic", "0x9a7A75E66B325a3BD46973B2b57c9b8d9D26a621");
	uniswap_v1_dexes.insert("shib", "0x5D9b6020EeF51fCB09390Bb4E07591f73c805065");

	let mut tk: Option<&str> = None;
	let mut dex_addr: Option<&str> = None;

	for (token, address) in &uniswap_v1_dexes {
		if let Some(addr) = successful_tx.to {
			if addr == address.parse().unwrap() {
				println!("Using uniswap v1");
				println!("DEX of token: {}", token);
				tk = Some(token);
				dex_addr = Some(address);
				uniswap_v1(successful_tx, successful_receipt, tokens, uniswap_v1_dexes, abi_paths.get("uniswap_v1").unwrap()).await;
				break;
			}
		} 
	}

	if tk.is_none() && dex_addr.is_none() {
		println!("Unknown contract");
	}
}

async fn uniswap_v1(tx: Transaction, receipt: TransactionReceipt, tokens: HashMap<&str, &str>, dexes: HashMap<&str, &str>, abi_path: &str) {
    // Read and parse the contract ABI
    let contract_abi = std::fs::read_to_string(abi_path).expect("Failed to read contract ABI");
    let contract = ethabi::Contract::load(contract_abi.as_bytes()).expect("Failed to parse contract ABI");

    // Extract and decode the input data from the transaction
    let input_data = tx.input.0.as_slice();
    let function = contract.functions()
        .find(|function| function.short_signature() == input_data[..4])
        .expect("Failed to find called function in contract ABI");
    let params = function.decode_input(&input_data[4..]).expect("Failed to decode input data");

    // Print the called function and its input parameter names and values
    println!("Called function: {}", function.name);
		let mut min_tokens: U256 = U256::zero();
		let mut recipient: String = "".to_string();
		let mut tokens_bought: U256 = U256::zero();
		let mut tokens_sold: U256 = U256::zero();
		let mut min_eth: U256 = U256::zero();
		let mut eth_bought: U256 = U256::zero();
		let mut max_tokens: U256 = U256::zero();
		let mut token_addr: String = "".to_string();
		let mut token_addr_name: Option<&str> = None;
		let mut min_tokens_bought: U256 = U256::zero();
		let mut max_tokens_sold: U256 = U256::zero();
		let mut exchange_addr: String = "".to_string();
		let mut exchange_addr_name: Option<&str> = None;
    for (param, value) in function.inputs.iter().zip(params) {
				// println!("{}: {:?}", param.name, value);
				if param.name == "min_tokens" {
					min_tokens = value.clone().into_uint().unwrap();
				}
				if param.name == "recipient" {
					recipient = value.clone().into_address().unwrap().to_string();
				}
				if param.name == "tokens_bought" {
					tokens_bought = value.clone().into_uint().unwrap();
				}
				if param.name == "tokens_sold" {
					tokens_sold = value.clone().into_uint().unwrap();
				}
				if param.name == "min_eth" {
					min_eth = value.clone().into_uint().unwrap();
				}
				if param.name == "eth_bought" {
					eth_bought = value.clone().into_uint().unwrap();
				}
				if param.name == "max_tokens" {
					max_tokens = value.clone().into_uint().unwrap();
				}
				if param.name == "token_addr" {
					for (token, address) in &tokens {
						if address.to_owned() == value.clone().into_address().unwrap().to_string() {
							token_addr_name = Some(token);
							break;
						}
					}
					if token_addr_name.is_none() {
						println!("Error: trading with illegal token");
						std::process::exit(1);
					}
					token_addr = value.clone().into_address().unwrap().to_string();
				}
				if param.name == "min_tokens_bought" {
					min_tokens_bought = value.clone().into_uint().unwrap();
				}
				if param.name == "max_tokens_sold" {
					max_tokens_sold = value.clone().into_uint().unwrap();
				}
				if param.name == "exchange_addr" {
					for (token, address) in &dexes {
						if address.to_owned() == value.clone().into_address().unwrap().to_string() {
							exchange_addr_name = Some(token);
							break;
						}
					}
					if exchange_addr_name.is_none() {
						println!("Error: trading with illegal token");
						std::process::exit(1);
					}
					exchange_addr = value.clone().into_address().unwrap().to_string();
				}
    }

		match function.name.as_str() {
			"__default__" => {
				println!("Bought {} WEI worth of tokens", tx.value);
			},
			"ethToTokenSwapInput" => {
				println!("Bought {} WEI worth of tokens, with a minimum of {} tokens", tx.value, min_tokens);
			},
			"ethToTokenTransferInput" => {
				println!("Bought {} WEI worth of tokens, with a minimum of {} tokens, for the recipient {}", tx.value, min_tokens, recipient);
			},
			"ethToTokenSwapOutput" => {
				println!("Bought {} tokens, with a maximum of {} WEI", tokens_bought, tx.value);
			},
			"ethToTokenTransferOutput" => {
				println!("Bought {} tokens, with a maximum of {} WEI, for the recipient {}", tokens_bought, tx.value, recipient);
			},
			"tokenToEthSwapInput" => {
				println!("Sold {} tokens, for a minimum of {} WEI", tokens_sold, min_eth);
			},
			"tokenToEthTransferInput" => {
				println!("Sold {} tokens, for a minimum of {} WEI, for the recipient {}", tokens_sold, min_eth, recipient);
			},
			"tokenToEthSwapOutput" => {
				println!("Sold {} WEI worth of tokens, for a maximum of {} tokens", eth_bought, max_tokens);
			},
			"tokenToEthTransferOutput" => {
				println!("Sold {} WEI worth of tokens, for a maximum of {} tokens, for the recipient {}", eth_bought, max_tokens, recipient);
			},
			"tokenToTokenSwapInput" => {
				println!("Sold {} tokens, with a minimum of {} {} tokens", tokens_sold, min_tokens_bought, token_addr_name.unwrap());
			},
			"tokenToTokenTransferInput" => {
				println!("Sold {} tokens, with a minimum of {} {} tokens, for the recipient {}", tokens_sold, min_tokens_bought, token_addr_name.unwrap(), recipient);
			},
			"tokenToTokenSwapOutput" => {
				println!("Bought {} {} tokens, with a maximum of {} tokens", tokens_bought, token_addr_name.unwrap(), max_tokens_sold);
			},
			"tokenToTokenTransferOutput" => {
				println!("Bought {} {} tokens, with a maximum of {} tokens, for the recipient {}", tokens_bought, token_addr_name.unwrap(), max_tokens_sold, recipient);
			},
			"tokenToExchangeSwapInput" => {
				println!("Sold {} tokens, with a minimum of {} {} tokens", tokens_sold, min_tokens_bought, exchange_addr_name.unwrap());
			},
			"tokenToExchangeTransferInput" => {
				println!("Sold {} tokens, with a minimum of {} {} tokens, for the recipient {}", tokens_sold, min_tokens_bought, exchange_addr_name.unwrap(), recipient);
			},
			"tokenToExchangeSwapOutput" => {
				println!("Bought {} {} tokens, with a maximum of {} tokens", tokens_bought, exchange_addr_name.unwrap(), max_tokens_sold);
			},
			"tokenToExchangeTransferOutput" => {
				println!("Bought {} {} tokens, with a maximum of {} tokens, for the recipient {}", tokens_bought, exchange_addr_name.unwrap(), max_tokens_sold, recipient);
			},
			_ => println!("Called function does not perform a trade"),
		}

		// Print the WEI value transacted by the message (if it exists)
		// println!("Value: {} WEI", tx.value);
}
