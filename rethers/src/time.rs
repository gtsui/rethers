use chrono::prelude::*;
use std::time::{UNIX_EPOCH, Duration};

pub fn current_time_str() -> String {
  return fmt_timestamp(current_time_secs())
}

pub fn current_time_secs() -> u64 {
  let time = std::time::SystemTime::now();
  time.duration_since(UNIX_EPOCH).unwrap().as_secs()  
}

pub fn fmt_timestamp(timestamp: u64) -> String {
  let d = UNIX_EPOCH + Duration::from_secs(timestamp);
  let datetime = DateTime::<Local>::from(d);
  datetime.format("%Y-%m-%d %H:%M:%S").to_string()  
}
