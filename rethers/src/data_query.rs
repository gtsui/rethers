use std::sync::Arc;
use std::fs::File;
use std::io::*;
use ethers::prelude::*;
use crate::*;



pub async fn run_event_query(
  provider_url: &str,
  addresses: Vec<H160>,
  topic: &str,
  start_block: u64,
  end_block: u64,
  chunk_size: u64
) {

  let result = _get_result_string(
    provider_url,
    addresses,
    topic,
    start_block,
    end_block,
    chunk_size
  ).await;

  _write_to_csv(&result);

  println!("Results stored in \\queries\\query.csv");
}

pub async fn run_event_query_with_header(
  provider_url: &str,
  addresses: Vec<H160>,
  topic: &str,
  start_block: u64,
  end_block: u64,
  chunk_size: u64,
  header: &str
) {

  let result = _get_result_string(
    provider_url,
    addresses,
    topic,
    start_block,
    end_block,
    chunk_size
  ).await;

  let result_with_header = header.to_string() + "\n" + &result;  
  _write_to_csv(&result_with_header);

  println!("Results stored in \\queries\\query.csv");
}


pub async fn _get_result_string(
  provider_url: &str,
  addresses: Vec<H160>,
  topic: &str,
  start_block: u64,
  end_block: u64,
  chunk_size: u64
) -> String {

  let provider = get_ws_provider(provider_url).await;

  let topic_hash = hash_event_signature(topic);
  
  let logs = get_logs_by_chunk(
    Arc::clone(&provider),
    addresses,
    vec![topic_hash],
    start_block,
    end_block,
    chunk_size
  ).await;

  let decoder = _topic_to_decoder(topic);

  let mut csv_string = "".to_string();
  for log in logs.into_iter() {
    let s = _format_log(log, decoder.clone());
    csv_string += &(s + "\n");
  }

  csv_string
}



fn _write_to_csv(content: &str) {
  let filename = "query.csv";
  let mut file = File::create(filename).unwrap();
  writeln!(&mut file, "{}", content).unwrap();
}

fn _topic_to_decoder(decoder_str: &str) -> Vec<LogType> {
  let start = decoder_str.find('(').unwrap() + 1;
  let end = decoder_str.find(')').unwrap();

  let s = &decoder_str[start..end];
  let v = s.split(",").map(|x| x.into()).collect::<Vec<&str>>();

  let mut decoder: Vec<LogType> = vec![];

  for log_type in v.into_iter() {
    if log_type == "bytes32" {
      decoder.push(LogType::H256);
    } else if log_type == "address" {
      decoder.push(LogType::H160);
    } else if log_type.contains("uint") {
      decoder.push(LogType::U256);
    } else {
      panic!("Unrecognized decoder symbol: {:?}", log_type);
    }
  }

  decoder
}

fn _format_log(log: Log, decoder: Vec<LogType>) -> String {
  let data = log.data.to_vec();
  let topics = &log.topics;
  let num_topics = topics.len() - 1; // Skip topic 0
  let (topic_decoder, data_decoder) = decoder.split_at(num_topics);
  let mut output = "".to_string();

  for (i, d) in topic_decoder.iter().enumerate() {
    if *d == LogType::H256 {
      let value = topics[i+1];
      output += &(value.to_string() + ",");
    } else if *d == LogType::H160 {
      let value = H160::from(topics[i+1]);
      output += &(format!("{:?}",value) + ",");
    } else if *d == LogType::U256 {
      let value = U256::from_big_endian(topics[i+1].as_bytes());
      output += &(value.to_string() + ",");
    }    
  }
  
  for (i, d) in data_decoder.iter().enumerate() {
    let start = i * 32;
    let end = (i+1) * 32;
    if *d == LogType::H256 {
      let value = H256::from_slice(&data[start..end]);
      output += &(value.to_string() + ",");
    } else if *d == LogType::H160 {
      let value = H160::from(H256::from_slice(&data[start..end]));
      output += &(format!("{:?}", value) + ",");
    } else if *d == LogType:: U256 {
      let value = U256::from(&data[start..end]);
      output += &(value.to_string() + ",");
    }        
  }
  output.pop(); // Remove trailing ','
  output
}
