use ethers::prelude::*;


pub async fn get_logs_by_chunk(  
  provider: &Provider<Ws>,
  addresses: Vec<H160>,
  topics: Vec<H256>,
  start_block: u64,
  end_block: u64,
  chunk_size: u64
) -> Vec<Log> {

  let mut current_block = start_block;

  let mut logs: Vec<Log> = vec![];
  
  while current_block < end_block {
    
    println!("Getting logs from {} to {}", current_block, current_block + chunk_size);
    
    let filter = create_filter(addresses.clone(), topics.clone(), current_block, current_block + chunk_size);

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
