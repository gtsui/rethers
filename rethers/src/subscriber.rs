#![allow(warnings, unused)]

use ethers::prelude::*;
use tokio::sync::mpsc::*;
use std::sync::*;
use crate::*;

pub async fn _subscribe_pending_txs(tx: Sender<BlockchainMessage>, provider: Arc<Provider<Ws>>) {

  tokio::spawn(async move {
    
    let mut stream = provider.subscribe_pending_txs().await.unwrap();
      
    while let Some(txn_hash) = stream.next().await {
      let maybe_txn = provider.get_transaction(txn_hash).await.unwrap_or_else(|_| None);
      if let Some(txn) = maybe_txn {
        let msg = BlockchainMessage::Txn(txn);
        tx.send(msg).await;
      }
    }
  });
  
}

pub async fn _subscribe_blocks(tx: Sender<BlockchainMessage>, provider: Arc<Provider<Ws>>) {
  tokio::spawn(async move {
    
    let mut stream = provider.subscribe_blocks().await.unwrap();
    
    while let Some(block) = stream.next().await {
      let msg = BlockchainMessage::Blk(block);
      tx.send(msg).await;
    }
    
  });
}
