use ethers_utils::*;

#[tokio::main]
async fn main() {

  let msg = format!("https://etherscan.io/tx/{}", "0x764e6df1bdcafc97b720e34095d199414a5567693b844a49022cf7318f6e0c06");
  
  telegram_alert(&env_key_str("TELEGRAM_BOT_TOKEN"), -403280594, &msg).await;
    
}
