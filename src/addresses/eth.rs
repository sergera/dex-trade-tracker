
fn tokens() -> HashMap<&str, &str> {
	let mut tokens: HashMap<&str, &str> = HashMap::new();
	tokens.insert("usdt", "0xdAC17F958D2ee523a2206206994597C13D831ec7");
	tokens.insert("usdc", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
	tokens.insert("busd", "0x4Fabb145d64652a948d72533023f6E7A623C7C53");
	tokens.insert("weth", "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
	tokens.insert("wbnb", "0x418D75f65a02b3D53B2418FB8E1fe493759c7605");
	tokens.insert("bnb", "0xB8c77482e45F1F44dE1745F52C74426C631bDD52");
	tokens.insert("matic", "0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0");
	tokens.insert("shib", "0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE");
	tokens
}

fn uniswap_v1_dexes() -> HashMap<&str, &str> {
	let mut uniswap_v1_dexes: HashMap<&str, &str> = HashMap::new();
	uniswap_v1_dexes.insert("usdt", "0xc8313c965C47D1E0B5cDCD757B210356AD0e400C");
	uniswap_v1_dexes.insert("usdc", "0x97deC872013f6B5fB443861090ad931542878126");
	uniswap_v1_dexes.insert("busd", "0x25C610eeE8f59768c26567c388986Aab3467a3E3");
	uniswap_v1_dexes.insert("bnb", "0x255e60c9d597dCAA66006A904eD36424F7B26286");
	uniswap_v1_dexes.insert("matic", "0x9a7A75E66B325a3BD46973B2b57c9b8d9D26a621");
	uniswap_v1_dexes.insert("shib", "0x5D9b6020EeF51fCB09390Bb4E07591f73c805065");
	uniswap_v1_dexes
}

fn uniswap_universal_router() -> String {
	"0xEf1c6E67703c7BD7107eed8303Fbe6EC2554BF6B".to_owned()
}

fn pancakeswap_smart_router() -> String {
	"0x13f4EA83D0bd40E75C8222255bc855a974568Dd4".to_owned()
}