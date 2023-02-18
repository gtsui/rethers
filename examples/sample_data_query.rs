#![allow(unused)]

use async_trait::*;
use rethers::*;
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {

  let address = str_to_H160("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"); //USDC ERC20
  let topic = "Transfer(address,address,uint256)";
  let provider_url = env_key_prefixed("WS_PROVIDER");   
  let header = "from,to,amount";
  
  run_event_query_with_header(
    &provider_url,
    vec![address],
    topic,
    16654465,
    16655465,
    100,
    header
  ).await;
    
}

