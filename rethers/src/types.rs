use ethers::prelude::*;

#[derive(Debug)]
pub enum BlockchainMessage {
  Txn(Transaction),
  Blk(Block<H256>)
}
