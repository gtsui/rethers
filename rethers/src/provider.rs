use std::thread;
use std::time::Duration;
use std::sync::Arc;
use ethers::prelude::*;
use async_recursion::async_recursion;
use crate::*;

pub async fn get_ws_provider(url: &str) -> Arc<Provider<Ws>> {
  println!("[{}] Connecting to blockchain provider...", fmt_timestamp(current_time()));
  let provider = _get_ws_provider_helper(url).await;
  println!("[{} Connected to blockchain provider", fmt_timestamp(current_time()));
  provider
}

#[async_recursion]
async fn _get_ws_provider_helper(url: &str) -> Arc<Provider<Ws>> {
  let result = Provider::<Ws>::connect(url).await;
  let provider = match result {
    Ok(provider) => {
      Arc::new(provider)
    },
    Err(_) => {
      println!("Error: Retrying to connect provider after 5 seconds...");
      thread::sleep(Duration::from_millis(5000));
      get_ws_provider(url).await
    }
  };
  provider
}

pub fn get_http_provider(url: &str) -> Provider<Http> {
  Provider::<Http>::try_from(url).unwrap()
}

pub async fn get_latest_block(provider: Arc<Provider<Ws>>) -> u64 {
  provider.get_block(BlockNumber::Latest).await.unwrap().unwrap().number.unwrap().as_u64()
}
