use ethers::prelude::*;
use crate::provider::*;

pub async fn get_logs_by_chunk(  
  provider: &Provider<Ws>,
  addresses: Vec<H160>,
  topics: Vec<H256>,
  start_block: u64,
  end_block: u64,
  chunk_size: u64
) -> Vec<Log> {

  let mut current_block = start_block;

  let latest_block = get_latest_block(&provider).await;
  
  let mut logs: Vec<Log> = vec![];

  if start_block > latest_block {
    panic!("start block greater than latest network block {}", latest_block);
  }

  let end_block = std::cmp::min(end_block, latest_block);
  
  while current_block < end_block {

    let current_start_block = current_block;
    let current_end_block = std::cmp::min(current_block + chunk_size - 1, end_block);
    
    println!("Getting logs from {} to {}", current_start_block, current_end_block);
    
    let filter = create_filter(
      addresses.clone(),
      topics.clone(),
      current_start_block,
      current_end_block
    );

    let mut log_chunk = provider.get_logs(&filter).await.unwrap();

    logs.append(&mut log_chunk);
    
    current_block = current_block + chunk_size;
  }
  
  logs
}


pub fn create_filter(
  addresses: Vec<H160>,
  topics: Vec<H256>,
  start_block: u64,
  end_block: u64
) -> Filter {
  
  Filter::new()
    .from_block(U64::from(start_block))
    .to_block(U64::from(end_block))
    .address(ValueOrArray::Array(addresses))
    .topic0(ValueOrArray::Array(topics))
             
}
