use chrono::prelude::*;
use std::time::{UNIX_EPOCH, Duration};

pub fn fmt_timestamp(timestamp: u64) -> String {
  let d = UNIX_EPOCH + Duration::from_secs(timestamp);
  let datetime = DateTime::<Local>::from(d);
  datetime.format("%Y-%m-%d %H:%M").to_string()  
}

pub fn current_time() -> u64 {
  let time = std::time::SystemTime::now();
  time.duration_since(UNIX_EPOCH).unwrap().as_secs()  
}
