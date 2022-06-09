use async_trait::*;
use ethers::prelude::*;
use crate::*;

#[async_trait]
pub trait MempoolListener {

  async fn listen(&mut self, provider_url: &str) {

    println!("Connecting to blockchain provider...");
    let provider = get_ws_provider(provider_url).await;
    println!("Connected to blockchain provider");

    println!("Starting...");
    self.on_start(&provider).await;

    let mut stream = provider.subscribe_pending_txs().await.unwrap();

    while let Some(tx_hash) = stream.next().await {
      let maybe_tx = provider.get_transaction(tx_hash).await.unwrap();
      if let Some(tx) = maybe_tx {
        self.on_msg(&provider, tx).await;
      }
    }
  }

  async fn on_start(&mut self, provider: &Provider<Ws>);

  async fn on_msg(&mut self, provider: &Provider<Ws>, tx: Transaction);  
}

#[async_trait]
pub trait BlockchainLogReader {

  async fn fetch_logs(
    &mut self,
    provider_url: &str,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    prior_blocks: u64,
    chunk_size: u64
  ) {

    println!("Connecting to blockchain provider...");
    let provider = get_ws_provider(provider_url).await;
    println!("Connected to blockchain provider");

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

  async fn fetch_logs_historical(
    &mut self,
    provider_url: &str,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    start_block: u64,
    end_block: u64,
    chunk_size: u64
  ) {

    println!("Connecting to blockchain provider...");
    let provider = get_ws_provider(provider_url).await;
    println!("Connected to blockchain provider");

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
  
  async fn on_fetched(&mut self, provider: &Provider<Ws>, logs: Vec<Log>);
}
