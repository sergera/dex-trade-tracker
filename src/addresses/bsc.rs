fn tokens() -> HashMap<&str, &str> {
	let mut tokens: HashMap<&str, &str> = HashMap::new();
	tokens.insert("usdt", "0x55d398326f99059ff775485246999027b3197955");
	tokens.insert("usdc", "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d");
	tokens.insert("busd", "0xe9e7cea3dedca5984780bafc599bd69add087d56");
	tokens.insert("wbnb", "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c");
	tokens.insert("weth", "0x4DB5a66E937A9F4473fA95b1cAF1d1E1D62E29EA");
	tokens.insert("bep20eth", "0x2170ed0880ac9a755fd29b2688956bd959f933f8");
	tokens.insert("matic", "0xcc42724c6683b7e57334c4e856f4c9965ed682bd");
	tokens
}

fn pancakeswap_smart_router() -> String {
	"0x13f4EA83D0bd40E75C8222255bc855a974568Dd4".to_owned()
}