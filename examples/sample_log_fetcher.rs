#[allow(unused)]

use async_trait::*;
use ethers_utils::*;
use ethers::prelude::*;

struct SampleLogFetcher {}

impl SampleLogFetcher {

  pub fn new() -> Self {
    SampleLogFetcher {}
  }
  
}

#[async_trait]
impl LogFetcher for SampleLogFetcher {
  
  async fn on_fetched(&mut self, provider: &Provider<Ws>, logs: Vec<Log>) {
    // do work here
    println!("{:?}", logs);    
  }
  
}


#[tokio::main]
async fn main() {
  
  let mut sample_log_fetcher = SampleLogFetcher::new();

  let address = env_key_prefixed_H160("UNISWAP_V2_FACTORY");
  
  let topic = hash_event_signature("PairCreated(address,address,address,uint256)");

  let provider_url = env_key_prefixed("WS_PROVIDER");

  sample_log_fetcher.fetch_logs(&provider_url, vec![address], vec![topic], 5000, 499).await;
  
}
