use async_trait::*;
use crate::*;

pub struct TestAlgo {

}

#[async_trait]
impl Algo for TestAlgo {

  async fn on_start<T: Algo>(&mut self, fw: &Framework<T>) {
    
  }

  async fn on_msg(&mut self, msg: &EventType) {
    match msg {
      EventType::PendingTx(tx) => {
        println!("{:?}", tx);
      },
      EventType::Block(block) => {

      },
      EventType::Log(log) => {

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