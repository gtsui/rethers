#![allow(unused)]

use async_trait::*;
use rethers::*;
use ethers::prelude::*;

struct SampleRethersLog {}

impl SampleRethersLog {

  pub fn new() -> Self {
    SampleRethersLog {}
  }

  pub async fn run(&mut self) {
    let address = str_to_H160("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"); //USDC ERC20
    let topic = hash_event_signature("Transfer(address,address,uint256)");
    let provider_url = env_key_prefixed("WS_PROVIDER");   
    self.fetch_logs_init_provider(&provider_url, vec![address], vec![topic], 5000, 500).await;    
  }
  
}

#[async_trait]
impl RethersLog for SampleRethersLog {
  
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
  
  let mut sample_rethers_log = SampleRethersLog::new();
  
  sample_rethers_log.run().await;
}
