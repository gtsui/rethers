use crate::*;

pub struct AlgoExecutor<T: Algo> {
  pub algos: Vec<T>,
  pub subscriber: AlgoFramework
}

impl<T> AlgoExecutor<T> where T: Algo {

  pub fn new() -> Self {
    let subscriber = AlgoFramework::new();
    AlgoExecutor {
      algos: vec![],
      subscriber
    }
  }

  pub fn add_algo(&mut self, algo: T) {
    self.algos.push(algo);
  }
  pub async fn run(&mut self) {    
    
    for algo in self.algos.iter_mut() {
      algo.on_start(&mut self.subscriber).await;
    }

    while let Some(msg) = self.subscriber.rx.recv().await {
      for algo in self.algos.iter_mut() {
        algo.on_msg(&self.subscriber, &msg).await;
      }
    }
  }
}