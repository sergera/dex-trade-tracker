Listening for a QuickAlert<br /> 
1- we setup a Chain for the QuickAlert, in this case the chain is the Ethereum Mainnet<br /> 

2- we input an Expression for the QuickAlert, in this case the expression is `(tx_to == '0x13f4ea83d0bd40e75c8222255bc855a974568dd4')` because I'm trying to get transactions made to the PancakeSwap Smart Router<br /> 
PS: if we wanted to get transfers made to escrow contracts, we would put `(tx_to == USDC_Contract_Address)` and check if the transfer was made to one of our escrow contract addresses.
We could also aggregate both in one QuickAlert by using an expression like `(tx_to == '0x13f4ea83d0bd40e75c8222255bc855a974568dd4') || (tx_to == USDC_Contract_Address)`<br /> 

3- we setup a Destination for the QuickAlert, this is a http server public URI for QuickNode to consume with an HTTP POST or GET request (it works with webhooks), since I'm using it locally I've setup [ngrok](https://ngrok.com/) with a free account, to forward a webhook request made to a public URI to my local service<br /> 

4- I've written a small service in Rust using warp that parses the request body into a `serde_json` value, and then parses that into a `web3::types::Transaction`<br /> 

From here we could parse the inputs using the Smart Router ABI, and figure out what trade was made, in what version, what are the assets, what is the fee (if applicable), and what is the path (depends on function / version)<br /> 