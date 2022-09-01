#![allow(warnings, unused)]

use ethers::prelude::*;
use tokio::sync::mpsc::*;
use std::sync::*;
use crate::*;

pub(crate) async fn _subscribe_pending_txs(provider: Arc<Provider<Ws>>, tx: Sender<BlockchainMessage>) {

  tokio::spawn(async move {
    
    let mut stream = provider.subscribe_pending_txs().await.unwrap();
      
    while let Some(txn_hash) = stream.next().await {
      let maybe_txn = provider.get_transaction(txn_hash).await.unwrap_or_else(|_| None);
      if let Some(txn) = maybe_txn {
        let msg = BlockchainMessage::PendingTx(txn);
        tx.send(msg).await;
      }
    }
  });
  
}

pub(crate) async fn _subscribe_blocks(provider: Arc<Provider<Ws>>, tx: Sender<BlockchainMessage>) {

  tokio::spawn(async move {
    
    let mut stream = provider.subscribe_blocks().await.unwrap();    

    while let Some(block_header) = stream.next().await {
      let maybe_block_with_txs = _block_header_to_block(Arc::clone(&provider), block_header).await;      
      if let Some(block_with_txs) = maybe_block_with_txs {
          tx.send(BlockchainMessage::BlockWithTxs(block_with_txs)).await;
      }                      
    }    
  });
}

async fn _block_header_to_block(provider: Arc<Provider<Ws>>, block_header: Block<H256>) -> Option<Block<Transaction>> {
  match block_header.hash {
    Some(block_hash) => {
      provider.get_block_with_txs(block_hash).await.unwrap_or_else(|_| None)   
    }
    _ => None
  }
}

pub(crate) async fn _subscribe_logs(provider: Arc<Provider<Ws>>, tx: Sender<BlockchainMessage>, filter: Filter) {

  tokio::spawn(async move {

    let mut stream = provider.subscribe_logs(&filter).await.unwrap();

    while let Some(log) = stream.next().await {
      tx.send(BlockchainMessage::Log(log)).await;
    }
    
  });  
}
