#![allow(warnings, unused)]

use async_trait::*;
use ethers::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::*;


#[async_trait]
pub trait RethersFramework {

  async fn on_start(&mut self, provider: Arc<Provider<Ws>>);

  async fn on_msg(&mut self, provider: Arc<Provider<Ws>>, msg: BlockchainMessage);    

  async fn run(&mut self, provider_url: &str, opts: FrameworkOptions) {

    let provider = get_ws_provider(provider_url).await;
    
    self.on_start(Arc::clone(&provider)).await;
    
    let (tx, mut rx) = mpsc::channel(1024);

    if opts.subscribe_pending_txs {
      let tx_pending_txs = tx.clone();
      _subscribe_pending_txs(Arc::clone(&provider), tx_pending_txs).await;
    }

    if opts.subscribe_blocks {
      let tx_blocks = tx.clone();
      _subscribe_blocks(Arc::clone(&provider), tx_blocks).await;
    }

    for filter in opts.log_filters {
      let tx_logs = tx.clone();
      _subscribe_logs(Arc::clone(&provider), tx_logs, filter).await;
    }
    
    while let Some(msg) = rx.recv().await {
      self.on_msg(Arc::clone(&provider), msg).await;
    }
  }
  
}


#[async_trait]
pub trait RethersLog {

  async fn on_fetched(&mut self, provider: Arc<Provider<Ws>>, logs: Vec<Log>);
  
  async fn fetch_logs(
    &mut self,
    provider: Arc<Provider<Ws>>,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    prior_blocks: u64,
    chunk_size: u64
  ) {

    let latest_block = get_latest_block(Arc::clone(&provider)).await;
    
    let logs = get_logs_by_chunk(
      Arc::clone(&provider),
      addresses,
      topics,
      latest_block - prior_blocks,
      latest_block,
      chunk_size
    ).await;
    
    self.on_fetched(Arc::clone(&provider), logs).await;
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
      provider,
      addresses,
      topics,
      prior_blocks,
      chunk_size
    ).await;
  }

  async fn fetch_logs_historical(
    &mut self,
    provider: Arc<Provider<Ws>>,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    start_block: u64,
    end_block: u64,
    chunk_size: u64
  ) {

    let logs = get_logs_by_chunk(
      Arc::clone(&provider),
      addresses,
      topics,
      start_block,
      end_block,
      chunk_size
    ).await;

    self.on_fetched(Arc::clone(&provider), logs).await; 
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
      provider,
      addresses,
      topics,
      start_block,
      end_block,
      chunk_size
    ).await;
  }

  
}
