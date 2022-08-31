#![allow(warnings, unused)]

use async_trait::*;
use ethers::prelude::*;
use tokio::sync::*;
use crate::*;


#[async_trait]
pub trait RethersFramework {

  async fn on_start(&mut self, provider: &Provider<Ws>);

  async fn on_msg(&mut self, provider: &Provider<Ws>, msg: BlockchainMessage);    

  async fn run(&mut self, provider_url: &str) {

    let provider = get_ws_provider(provider_url).await;

    self.on_start(&provider).await;
    
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    
    _subscribe_pending_txs(tx, String::from(provider_url)).await;
    
    _subscribe_blocks(tx2, String::from(provider_url)).await;
    
    while let Some(msg) = rx.recv().await {
      self.on_msg(&provider, msg).await;
    }
  }
  
}


#[async_trait]
pub trait RethersLog {

  async fn on_fetched(&mut self, provider: &Provider<Ws>, logs: Vec<Log>);
  
  async fn fetch_logs(
    &mut self,
    provider: &Provider<Ws>,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    prior_blocks: u64,
    chunk_size: u64
  ) {

    let latest_block = get_latest_block(&provider).await;
    
    let logs = get_logs_by_chunk(
      &provider,
      addresses,
      topics,
      latest_block - prior_blocks,
      latest_block,
      chunk_size
    ).await;
    
    self.on_fetched(&provider, logs).await;     
  }

  
  async fn fetch_logs_init_provider(
    &mut self,
    provider_url: &str,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    prior_blocks: u64,
    chunk_size: u64
  ) {
    
    let provider = get_ws_provider(provider_url).await;
    self.fetch_logs(
      &provider,
      addresses,
      topics,
      prior_blocks,
      chunk_size
    ).await;
  }

  async fn fetch_logs_historical(
    &mut self,
    provider: &Provider<Ws>,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    start_block: u64,
    end_block: u64,
    chunk_size: u64
  ) {

    let logs = get_logs_by_chunk(
      &provider,
      addresses,
      topics,
      start_block,
      end_block,
      chunk_size
    ).await;

    self.on_fetched(&provider, logs).await; 
  }

  async fn fetch_logs_historical_init_provider(
    &mut self,
    provider_url: &str,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    start_block: u64,
    end_block: u64,
    chunk_size: u64
  ) {

    let provider = get_ws_provider(provider_url).await;

    self.fetch_logs_historical(
      &provider,
      addresses,
      topics,
      start_block,
      end_block,
      chunk_size
    ).await;
  }
  
}
