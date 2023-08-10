#![allow(unused)]

use async_trait::*;
use rethers::*;

#[tokio::main]
async fn main() {

  let test_algo = TestAlgo::new();

  let mut ae: AlgoExecutor<TestAlgo> = AlgoExecutor::new();
  ae.add_algo(test_algo);

  ae.run().await;
}

pub struct TestAlgo {

}

#[async_trait]
impl Algo for TestAlgo {

  async fn on_start(&mut self, af: &mut AlgoFramework) {
        
    println!("Starting...");        

    let provider_url = env_key_str("ETHEREUM_PROVIDER");

    af.subscribe_pending_txs(&provider_url).await;

    af.subscribe_logs(
      &provider_url,
      str_to_H160("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"), // USDC
      "Transfer(address,address,uint256)"
    ).await;
  }

  async fn on_msg(&mut self, af: &AlgoFramework, msg: &EventType) {
    match msg {
      EventType::PendingTx(tx) => {
        //println!("{:?}", tx);
      },
      EventType::Block(block) => {

      },
      EventType::Log(log) => {
        println!("{:?}", log);
      }
    }
  }

}

impl TestAlgo {
  pub fn new() -> Self {
    TestAlgo {

    }
  }
}