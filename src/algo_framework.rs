use std::{collections::HashMap, sync::Arc};
use async_recursion::async_recursion;
use tokio::sync::{mpsc, mpsc::*};
use ethers::prelude::*;
use crate::*;

pub struct AlgoFramework {
  pub providers: HashMap<String, Arc<Provider<Ws>>>,
  pub tx: Sender<EventType>,
  pub rx: Receiver<EventType>
}

impl AlgoFramework {

  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel(1024);
    AlgoFramework {
      providers: HashMap::new(),
      tx,
      rx
    }
  }

  pub async fn add_provider(&mut self, provider_url: &str) {
    let provider = get_ws_provider(provider_url).await;
    self.providers.insert(provider_url.to_string(), Arc::clone(&provider));
  }

  #[async_recursion]
  pub async fn subscribe_pending_txs(&mut self, provider_url: &str) {
    let maybe_provider = self.providers.get(provider_url);
    match maybe_provider {
      Some(provider) => {
        _subscribe_pending_txs(Arc::clone(&provider), self.tx.clone()).await;
      },
      None => {
        self.add_provider(provider_url).await;
        self.subscribe_pending_txs(provider_url).await;
      }
    }
  }

  #[async_recursion]
  pub async fn subscribe_blocks(&mut self, provider_url: &str) {
    let maybe_provider = self.providers.get(provider_url);
    match maybe_provider {
      Some(provider) => {
        _subscribe_blocks(Arc::clone(&provider), self.tx.clone()).await;
      },
      None => {
        self.add_provider(provider_url).await;
        self.subscribe_blocks(provider_url).await;        
      }
    }
  }

  #[async_recursion]
  pub async fn subscribe_logs(&mut self, provider_url: &str, contract_address: H160, topic: &str) {
    let maybe_provider = self.providers.get(provider_url);
    match maybe_provider {
      Some(provider) => {        
        let filter = create_filter(vec![contract_address], vec![event_signature(topic)]);
        _subscribe_logs(Arc::clone(&provider), self.tx.clone(), filter).await;
      },
      None => {
        self.add_provider(provider_url).await;
        self.subscribe_logs(provider_url, contract_address, topic).await;
      }
    }
  }
  

}
