use ethers::prelude::*;
use crate::provider::*;
use crate::time::*;

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


#[derive(PartialEq,Eq)]
pub enum LogType {
  H160,
  H256,
  U256
}

pub async fn print_logs(provider: &Provider<Ws>, logs: Vec<Log>, data_decoder: Vec<(LogType, &str)>, topic_decoder: Vec<(LogType, &str)>) {

  for log in logs.iter() {
    let data = log.data.to_vec();
    let topics = &log.topics;
    
    // log metadata
    let address = log.address;
    let txhash = log.transaction_hash.unwrap();
    let tx = provider.get_transaction(txhash.clone()).await.unwrap().unwrap();
    let from = tx.from;
    let to = tx.to;
    let block_number = tx.block_number.unwrap();
    let block = provider.get_block(block_number).await.unwrap().unwrap();
    let datetime = fmt_timestamp(block.timestamp.low_u64());

    println!("------EVENT------");
    println!("{:?}", datetime);
    println!("txhash: {:?}", txhash);    
    println!("address: {:?}", address);
    println!("from: {:?}", from);
    println!("to: {:?}", to);

    for (i, decoder) in topic_decoder.iter().enumerate() {
      // Skip topic 0
      if decoder.0 == LogType::H256 {
        let value = topics[i+1];
        println!("{}: {:?}", decoder.1, value);
      } else if decoder.0 == LogType::H160 {
        let value = H160::from(topics[i+1]);
        println!("{}: {:?}", decoder.1, value);
      } else if decoder.0 == LogType::U256 {
        let value = U256::from_little_endian(topics[i+1].as_bytes());
        println!("{}: {:?}", decoder.1, value);
      }
    }

    for (i, decoder) in data_decoder.iter().enumerate() {
      let start = i * 32;
      let end = (i+1) * 32;    
      if decoder.0 == LogType::H256 {
        let value = H256::from_slice(&data[start..end]);
        println!("{}: {:?}", decoder.1, value);
      } else if decoder.0 == LogType::H160 {
        let value = H160::from(H256::from_slice(&data[start..end]));
        println!("{}: {:?}", decoder.1, value);
      }else if decoder.0 == LogType::U256 {
        let value = U256::from(&data[start..end]);
        println!("{}: {:?}", decoder.1, value);
      }
    }    
    println!("-----------------");
  }  
}
