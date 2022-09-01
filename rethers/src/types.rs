use ethers::prelude::*;

#[derive(Debug)]
pub enum BlockchainMessage {
  PendingTx(Transaction),
  BlockWithTxs(Block<Transaction>)
}
