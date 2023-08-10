#[derive(Debug)]
pub enum EventType {
  PendingTx(String, ethers::types::Transaction),
  Block(String, ethers::types::Block<ethers::types::Transaction>),
  Log(String, ethers::types::Log)
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum LogType {
  H160,
  H256,
  U256
}
