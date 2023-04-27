use std::env;
use std::fs;
use std::collections::HashMap;

use regex::Regex;

use ethabi::{Contract, Token, Function, Param, ParamType};
use ethabi::ethereum_types::{U256};
use serde_json;
use serde_json::Value;

use web3::futures::Future;
use web3::types::{BlockNumber, FilterBuilder};
use web3::types::{Transaction, TransactionReceipt, TransactionId};
use web3::types::{Address, Bytes, H256, H160};

// Trying to integrate with pancakeswap

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
	let rpc_provider_url = config["bsc-rpc-provider-url"].as_str().expect("Error: Failed to get rpc-provider-url");

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

	println!("TX: {:?}", successful_tx);

	let mut abi_paths: HashMap<&str, &str> = HashMap::new();
	abi_paths.insert("pancakeswap_router", "./abi/eth/pancakeswap/smart_router_v3.json");
	abi_paths.insert("weth", "./abi/eth/weth.json");
	abi_paths.insert("usdc", "./abi/eth/usdc.json");
	abi_paths.insert("bnb", "./abi/eth/bnb.json");
	abi_paths.insert("fiat_token_proxy", "./abi/eth/fiat_token_proxy.json");

	// Read and parse the contract ABI
	let contract_abi = std::fs::read_to_string(abi_paths.get("pancakeswap_router").unwrap()).expect("Failed to read contract ABI");
	let contract = ethabi::Contract::load(contract_abi.as_bytes()).expect("Failed to parse contract ABI");

	// Extract and decode the input data from the transaction
	let input_data = successful_tx.input.0.as_slice();
	let function = contract.functions()
	.find(|function| function.short_signature() == input_data[..4])
	.expect("Failed to find called function in contract ABI");
	let params = function.decode_input(&input_data[4..]).expect("Failed to decode input data");

	println!("Called function: {}", function.name);
	println!("With value: {}", successful_tx.value);
	println!("With params: {:?}", params);

	let mut multicall_functions: Vec<&Function> = Vec::new();
	let mut multicall_inputs_encoded: Vec<Vec<u8>> = Vec::new();
	for (param, value) in function.inputs.iter().zip(params) {
		// println!("{}: {:?}", param.name, value);
		if param.name == "data" {
			for token in value.into_array().unwrap() {
				// print_token(token);
				let bytes = token.into_bytes().unwrap();
				multicall_functions.push(
						contract.functions()
								.find(|function| function.short_signature() == bytes[..4])
								.expect("Failed to find called function in contract ABI"),
				);
				multicall_inputs_encoded.push(
					bytes[4..].to_vec()
				);
			}
		}
	}

	for i in 0..multicall_functions.len() {
		println!("Called function: {}", multicall_functions[i].name);
		println!("With params: {:?}", multicall_functions[i].decode_input(&multicall_inputs_encoded[i]).expect("Failed to decode input data"));
	}

	// let types = vec![
	// 	ParamType::Address,
	// 	ParamType::Uint(256),
	// 	ParamType::Array(Box::new(ParamType::Bytes)),
	// ];

	// let mut tokens: Vec<Vec<Token>> = Vec::new();
	// for bytes in data {
	// 	tokens.push(ethabi::decode(&types, &bytes).unwrap());
	// }
	
	// let mut target: String;
	// let mut gasLimit: U256;
	// let mut callData: Vec<u8>;
	// for tks in tokens {
	// 	target = format!("0x{}", hex::encode(tks[0].clone().into_address().unwrap().as_bytes()));
	// 	gasLimit = tks[1].clone().into_uint().unwrap();
	// 	callData = tks[2].clone().into_bytes().unwrap();
	// 	println!("target: {}", target);
	// 	println!("gasLimit: {}", gasLimit);
	// 	println!("callData: {:?}", callData);
	// }



	// let mut commands: Vec<u8> = Vec::new();
	// let mut inputs: Vec<Vec<u8>> = Vec::new();
	// for (param, value) in function.inputs.iter().zip(params) {
	// 	println!("{}: {:?}", param.name, value);
	// 	if param.name == "commands" {
	// 		for byte in value.into_bytes().unwrap() {
	// 			commands.push(byte);
	// 		}
	// 	} else if param.name == "inputs" {
	// 		for bytes in value.into_array().unwrap() {
	// 			inputs.push(bytes.into_bytes().unwrap());
	// 		}
	// 	}
	// }


	// let contract_addr = successful_tx.to.unwrap();
	// match contract_addr {
		
	// } 
}

fn print_token(token: Token) {
	match token {
			Token::Address(_) => println!("Address"),
			Token::FixedBytes(_) => println!("FixedBytes"),
			Token::Bytes(_) => println!("Bytes"),
			Token::Int(_) => println!("Int"),
			Token::Uint(_) => println!("Uint"),
			Token::Bool(_) => println!("Bool"),
			Token::String(_) => println!("String"),
			Token::FixedArray(_) => println!("FixedArray"),
			Token::Array(_) => println!("Array"),
			Token::Tuple(_) => println!("Tuple"),
	}
}