#![allow(unused)]

use async_trait::*;
use std::sync::{Arc};
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
      BlockchainMessage::Txn(tx) => {
        println!("{:?}", tx);
      },
      BlockchainMessage::Blk(b) => {
        println!("{:?}", b);
      }
    }

  }
}


#[tokio::main]
async fn main() {

  let config = Config::new();
  
  let mut sample_rethers_framework = SampleRethersFramework::new();

  sample_rethers_framework.run(&config.PROVIDER_URL).await;

}
