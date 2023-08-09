use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::*;
use ethers::prelude::*;
use crate::*;

pub struct Framework<T: Algo> {
  pub algo: T,
  pub providers: HashMap<String, Arc<Provider<Ws>>>,
  tx: Sender<EventType>,
  rx: Receiver<EventType>
}

impl<T> Framework<T> where T: Algo {

  pub fn new(algo: T) -> Self {
    let (tx, mut rx) = mpsc::channel(1024);
    Framework {
      algo,
      providers: HashMap::new(),
      tx,
      rx
    }
  }

  pub async fn add_provider(&mut self, provider_url: &str) {
    let provider = get_ws_provider(provider_url).await;
    self.providers.insert(provider_url.to_string(), Arc::clone(&provider));
  }

  pub async fn subscribe_pending_txs(&mut self, provider_url: &str) {
    let maybe_provider = self.providers.get(provider_url);
    match maybe_provider {
      Some(provider) => {
        _subscribe_pending_txs(Arc::clone(&provider), self.tx.clone()).await;
      },
      None => {
        let provider = get_ws_provider(provider_url).await;
        self.providers.insert(provider_url.to_string(), Arc::clone(&provider));
        _subscribe_pending_txs(Arc::clone(&provider), self.tx.clone()).await;
      }
    }
  }

  pub async fn run(&self) {

    self.algo.on_start(self).await;
    
    while let Some(msg) = self.rx.recv().await {
        //self.algo.on_msg(&msg);
    }
  }
}