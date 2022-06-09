#![allow(unused)]

use async_trait::*;
use ethers_utils::*;
use ethers::prelude::*;

struct SampleMempoolListener {}

impl SampleMempoolListener {

  pub fn new() -> Self {
    SampleMempoolListener {}
  }
  
}

#[async_trait]
impl MempoolListener for SampleMempoolListener {

  async fn on_start(&mut self, provider: &Provider<Ws>) {
    // do work here
    println!("Write initializing code here...");
  }

  async fn on_msg(&mut self, provider: &Provider<Ws>, tx: Transaction) {
    // do work here
    println!("{:?}", tx);
  }
  
}

#[tokio::main]
async fn main() {

  let mut sample_mempool_listener = SampleMempoolListener::new();

  let provider_url = env_key_prefixed("WS_PROVIDER");
  
  sample_mempool_listener.listen(&provider_url).await;
  
}
