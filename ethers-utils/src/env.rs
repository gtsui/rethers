use ethers::prelude::*;
use crate::*;

pub fn env_key_prefixed(key: &str) -> String {  
  let key = dotenv::var("ENV").unwrap() + "_" + key;
  dotenv::var(key).unwrap()
}

pub fn env_key_prefixed_U256(key: &str) -> U256 {
  let prefixed_key = env_key_prefixed(key);
  str_to_U256(&prefixed_key)
}

pub fn env_key_prefixed_H160(key: &str) -> H160 {
  let prefixed_key = env_key_prefixed(key);
  str_to_H160(&prefixed_key)
}

pub fn env_key_U256(key: &str) -> U256 {
  let key = dotenv::var(key).unwrap();
  str_to_U256(&key)
}

pub fn env_key_H160(key: &str) -> H160 {
  let key = dotenv::var(key).unwrap();
  key.parse::<H160>().unwrap()
}
