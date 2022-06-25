use teloxide::{prelude::*, types::Recipient};

pub async fn telegram_alert(token: &str, chat_id: i64, alert: &str) {
  let bot = Bot::new(token).auto_send();
  let id = Recipient::Id(ChatId(chat_id));
  let result = bot.send_message(id, alert).await;

  match result {
    Ok(msg) => {
      println!("Telegram sent: {:?}", msg.text());
    },
    Err(e) => {
      println!("Telegram Error: {:?}", e);
    }
  }
}
