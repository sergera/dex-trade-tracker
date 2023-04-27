Tracking Swaps in PancakeSwap<br /> 

This is test code to prove that a PancakeSwap trade can be tracked with the knowledge of the transaction id, in a way that retrieves the trading pair and relevant values, it is not in any way a complete working program, or near the quality standards for a demo (much less for production code), and lacks even basic features such as error handling.<br /> 

Firstly, PancakeSwap is deployed on both Ethereum and BSC, so there should be RPC provider URLs for both in the `config/config.json` file, and this test version of the code only queries one blockchain, so line 36 should be adapted to either get the `eth-rpc-provider-url` or `bsc-rpc-provider-url` depending on which blockchain ran the transaction to be queried.<br /> 

```sh
cargo run --bin new 'TRANSACTION_ID'
```

Upon receiving a transaction hash, the first order of business is to know if the transaction in fact ran, or if was reverted, or is pending.<br /> 

To do that we query the blockchain for the `transaction` and it's `receipt` through the RPC provider, if no `transaction` is returned, or if there is no `receipt` it doesn't exist, if the transaction is missing it's `block_number`, it is pending, if they are both present and the `receipt.status` is `0`, it was reverted, and if the `receipt.status` is `1`, it ran successfully.<br /> 

With the knowledge that this is in fact a transaction that ran successfully in the desired blockchain, we then check the address of the caller, which is in `transaction.from`, that enables us to filter for the callers we want to track.<br />

For the transaction to be a token trade in PancakeSwap, it must have called a function in the PancakeSwap Smart Router, therefore the transaction must be addressed to one of the contract addresses below, that can be checked with `transaction.to`.<br /> 

Smart Router Ethereum Address: 0x13f4EA83D0bd40E75C8222255bc855a974568Dd4<br />
Smart Router BSC Address: 0x13f4EA83D0bd40E75C8222255bc855a974568Dd4<br />
Smart Router Goerli Address: 0x9a489505a00cE272eAa5e07Dba6491314CaE3796<br />
Smart Router BSC testnet Address: 0x9a489505a00cE272eAa5e07Dba6491314CaE3796<br />

It is important to note that, even though all token trades go through the Smart Router, there are other contracts that can be called from the PancakeSwap swap interface, e.g. if the user only attempts to wrap or unwrap a native token PancakeSwap will make a call directly to the wETH / wBNB contract's `deposit` or `withdraw`functions, and before a user can trade any fungible token on the platform, they will need to authorize the Smart Router to move the specific token funds for them, PancakeSwap does this by calling the ERC-20 contract's `approve` function for that specific token.<br />

With this in mind, we must keep track of the addresses for wETH and wBNB in each blockchain.<br />

wETH Ethereum Address: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2<br />
wETH Goerli Address: 0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6<br />

wBNB BSC Address: 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c<br />
wBNB BSC testnet Address: 0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd<br />

We must also keep track of the contract addresses for all whitelisted tokens in each blockchain, below are the addresses for USDC and CAKE as an example.<br />

USDC Ethereum Address: 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48<br />
USDC BSC Address: 0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d<br />
USDC Goerli Address: 0x07865c6E87B9F70255377e024ace6630C1Eaa37F<br />

CAKE Ethereum Address: 0x152649eA73beAb28c5b49B26eb48f7EAD6d4c898<br />
CAKE BSC Address: 0x0e09fabb73bd3ade0a17ecc321fd13a19e81ce82<br />
CAKE BSC testnet Address: 0xFa60D973F7642B748046464e165A65B7323b0DEE<br />

With the knowledge that the transaction is successful, made my an actively tracked account, and that the contract called is allowed, it is now necessary to retrieve the exact function that was called and the values that were sent as parameters.<br />

To interpret calls to any contract, it is necessary for the code to have the specific contract's ABI. ABI stands for `Application Binary Interface` and it is a standard that defines how to communicate a smart contract's data structures and functions universally, it uses the widely popular JSON (JavaScript Object Notation) format.<br /> 

ABIs can be downloaded directly from the contract's page on a blockchain explorer (e.g. etherscan, bscscan) under the `Contract` tab, and this project contains several downloaded ABIs that can be found in the `/abi` directory.<br /> 

The process of using the ABI to parse contract calls is as follows:<br />
1- If applicable, retrieve the value of native tokens sent in the transaction from `transaction.value`. <br />
2- Retrieve contract call information from `transaction.input`, this field contains a byte array with the signature of the function called in the first 4 bytes `transaction.input[..4]` and the parameter values sent to the function in the remaining bytes `transaction.input[4..]`.<br />
3- Load contract functions contained in the ABI, and retrieve the `function` which's signature matches the signature retrieved from `transaction.input[..4]`.<br />
4- Decode parameter values from `transaction.input[4..]` using the function retrieved from the ABI.<br />
5- Retrieve parameter names from `function.inputs` and match them with decoded parameter values.<br />

After this process, we now have the caller, the native token value sent, the contract that was called, the function that was called in the contract, and the parameter names matched with their values that were sent in the function call.<br /> 
