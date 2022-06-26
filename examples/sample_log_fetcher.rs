#![allow(unused)]

use async_trait::*;
use ethers_utils::*;
use ethers::prelude::*;

struct SampleLogFetcher {}

impl SampleLogFetcher {

  pub fn new() -> Self {
    SampleLogFetcher {}
  }

  pub async fn run(&mut self) {
    let address = env_key_prefixed_H160("ERC20");
    let topic = hash_event_signature("Transfer(address,address,uint256)");
    let provider_url = env_key_prefixed("WS_PROVIDER");   
    self.fetch_logs_init_provider(&provider_url, vec![address], vec![topic], 5000, 500).await;    
  }
  
}

#[async_trait]
impl LogFetcher for SampleLogFetcher {
  
  async fn on_fetched(&mut self, provider: &Provider<Ws>, logs: Vec<Log>) {
    
    print_logs(
      provider,
      logs,
      vec![(LogType::U256, "amount")],
      vec![(LogType::H160, "from"), (LogType::H160, "to")]
    ).await;
  }
  
}


#[tokio::main]
async fn main() {
  
  let mut sample_log_fetcher = SampleLogFetcher::new();
  
  sample_log_fetcher.run().await;
}
