#![allow(non_snake_case)]

use rethers::*;

#[derive(Clone)]
pub struct Config {
  pub ENV: String,
  pub CHAIN_ID: u64,
  pub PROVIDER_URL: String,
}

impl Config {

  pub fn new() -> Self {
    Config {
      ENV: env_key_str("ENV"),
      CHAIN_ID: Self::env_to_chainid(&env_key_str("ENV")),
      PROVIDER_URL: env_key_prefixed("WS_PROVIDER")
    }
  }
  
  pub fn env_to_chainid(env: &str) -> u64 {
    if env == "MAINNET" {
      return 1;
    } else if env == "GOERLI" {
      return 5;
    } else if env == "BSC" {
      return 56;
    } else if env == "POLYGON" {
      return 137;
    } else if env == "FANTOM" {
      return 250;
    } else if env == "MOONBASE" {
      return 1287;
    } else if env == "MOONRIVER" {
      return 1285;
    } else if env == "MOONBEAM" {
      return 1284;
    } else if env == "AVAX" {
      return 43114;
    } else if env == "MUMBAI" {
      return 80001;
    } else {
      return 0;
    }
  }
}
