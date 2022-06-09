use async_trait::*;
use ethers_utils::*;
use ethers::prelude::*;

struct UniswapV2PairReader {}

impl UniswapV2PairReader {

  pub fn new() -> Self {
    UniswapV2PairReader {}
  }
  
}

#[async_trait]
impl BlockchainLogReader for UniswapV2PairReader {
  
  async fn on_fetched(&mut self, provider: &Provider<Ws>, logs: Vec<Log>) {
    println!("{:?}", logs);
  }
  
}


#[tokio::main]
async fn main() {
  
  let mut uniswap_pool_reader = UniswapV2PairReader::new();

  let address = env_key_prefixed_H160("FACTORY");
  
  let topic = hash_event_signature("PairCreated(address,address,address,uint256)");

  let provider_url = env_key_prefixed("WSS");

  uniswap_pool_reader.fetch_logs(&provider_url, vec![address], vec![topic], 5000, 499).await;
  
}
