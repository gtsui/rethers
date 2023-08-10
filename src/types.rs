#[derive(Debug)]
pub enum EventType {
  PendingTx(ethers::types::Transaction),
  Block(ethers::types::Block<ethers::types::Transaction>),
  Log(ethers::types::Log)
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum LogType {
  H160,
  H256,
  U256
}
