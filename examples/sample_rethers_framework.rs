#![allow(unused)]

use async_trait::*;
use std::sync::Arc;
use ethers::prelude::*;
use rethers::*;

struct SampleRethersFramework {}

impl SampleRethersFramework {

  pub fn new() -> Self {
    SampleRethersFramework {}
  }

  async fn pending_tx_handler(&mut self, provider: Arc<Provider<Ws>>, tx: Transaction) {
    println!("[{}] Pending Tx Hash: {:?}", current_time_str(), tx.hash);
  }

  async fn block_with_txs_handler(&mut self, provider: Arc<Provider<Ws>>, block: Block<Transaction>) {
    println!("[{}] Block Hash: {:?}", current_time_str(), block.hash);
  }

  async fn log_handler(&mut self, provider: Arc<Provider<Ws>>, log: Log) {
    println!("[{}] Log Tx Hash: {:?}", current_time_str(), log.transaction_hash);
  }
}

#[async_trait]
impl RethersFramework for SampleRethersFramework {

  async fn on_start(&mut self, provider: Arc<Provider<Ws>>) {
    println!("Write initializing code here...");
  }

  async fn on_msg(&mut self, provider: Arc<Provider<Ws>>, msg: EventType) {

    match msg {
      EventType::PendingTx(tx) => {
        self.pending_tx_handler(Arc::clone(&provider), tx).await;
      },
      EventType::Block(block) => {
        self.block_with_txs_handler(Arc::clone(&provider), block).await;
      },
      EventType::Log(log) => {
        self.log_handler(Arc::clone(&provider), log).await;
      }
    }

  }
}


#[tokio::main]
async fn main() {
  
  let mut sample_rethers_framework = SampleRethersFramework::new();

  let addresses = vec![str_to_H160("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")];

  let topics = vec![event_signature("Transfer(address,address,uint256)")];
  
  let filter = create_filter(addresses, topics);
  
  let opts = FrameworkOptions::new(
    true,
    true,
    vec![filter]
  );
  
  sample_rethers_framework.run("wss://...", opts).await;

}
