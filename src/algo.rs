#![allow(warnings, unused)]

use async_trait::*;
use ethers::prelude::*;
use std::sync::Arc;
use crate::*;

#[async_trait]
pub trait Algo {

  async fn on_start<T: Algo>(&mut self, fw: &Framework<T>);

  async fn on_msg(&mut self, msg: &EventType);    

}