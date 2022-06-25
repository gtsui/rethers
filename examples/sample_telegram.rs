use ethers_utils::*;

#[tokio::main]
async fn main() {

  let msg = format!("https://moonriver.moonscan.io/tx/{}", "0x7dbb4b1ed73ea2b6f4dd8ac27fc1f812c62fdb4a07715c2d09bde47af0bb7e07");
  
  telegram_alert(&env_key_str("TELEGRAM_BOT_TOKEN"), -403280594, &msg).await;

  
  // let bot = Bot::new(env_key_str("TELEGRAM_BOT_TOKEN")).auto_send();
  // let id = teloxide::types::Recipient::Id(ChatId(-403280594));

  // let res = bot.send_message(id, "whatsup madafaka").await.unwrap();

  // println!("{:?}", res);
  
  // teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {

  //   //println!("{:?}", message.chat.id);
  //   let id = teloxide::types::Recipient::Id(ChatId(-403280594));
  //   bot.send_message(id, "hello world!").await?;
  //   //bot.send_dice(id).await?;
  //   respond(())
  // })
  //   .await;
  
}
