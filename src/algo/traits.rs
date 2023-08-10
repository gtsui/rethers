use async_trait::*;
use crate::*;

#[async_trait]
pub trait Algo {

  async fn on_start(&mut self, af: &mut AlgoFramework);

  async fn on_msg(&mut self, af: &AlgoFramework, msg: &EventType);    

}