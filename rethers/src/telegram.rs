use std::thread;
use std::time::Duration;
use teloxide::{prelude::*, types::Recipient};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn telegram_alert(token: &str, chat_id: i64, alert: &str) {
  let bot = Bot::new(token).auto_send();
  let id = Recipient::Id(ChatId(chat_id));
  let result = bot.send_message(id, alert).await;

  match result {
    Ok(msg) => {
      println!("Telegram alert sent: {:?}", msg.text());
    },
    Err(_) => {
      println!("Telegram error: Retrying after 2 seconds...");
      thread::sleep(Duration::from_millis(2000));
      telegram_alert(token, chat_id, alert).await;
    }
  }
}
