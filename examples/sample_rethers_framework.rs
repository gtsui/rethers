#![allow(unused)]

use async_trait::*;
use std::sync::Arc;
use ethers::prelude::*;
use rethers::*;
use config::*;

struct SampleRethersFramework {}

impl SampleRethersFramework {

  pub fn new() -> Self {
    SampleRethersFramework {}
  }
  
}

#[async_trait]
impl RethersFramework for SampleRethersFramework {

  async fn on_start(&mut self, provider: Arc<Provider<Ws>>) {
    println!("Write initializing code here...");
  }

  async fn on_msg(&mut self, provider: Arc<Provider<Ws>>, msg: BlockchainMessage) {

    match msg {
      BlockchainMessage::PendingTx(tx) => {
        println!("Pending Tx Hash: {:?}", tx.hash);
      },
      BlockchainMessage::BlockWithTxs(b) => {
        println!("Block Hash: {:?}", b.hash);
      },
      BlockchainMessage::Log(l) => {
        println!("Log Tx Hash: {:?}", l.transaction_hash);
      }
    }

  }
}


#[tokio::main]
async fn main() {

  let config = Config::new();
  
  let mut sample_rethers_framework = SampleRethersFramework::new();

  let addresses = vec![str_to_H160("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")];

  let topics = vec![hash_event_signature("Transfer(address,address,uint256)")];
  
  let filter = create_filter(addresses, topics);
  
  let opts = FrameworkOptions::new(
    true,
    true,
    vec![filter]
  );
  
  sample_rethers_framework.run(&config.PROVIDER_URL, opts).await;

}
