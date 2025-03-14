use teloxide::prelude::*;
// use teloxide::utils::command::BotCommands;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN not found in .env");
    let bot = Bot::new(token);

    let handler = |bot: Bot, msg: Message| async move {
        if let Err(err) = bot.send_dice(msg.chat.id).await {
            log::error!("Error sending dice: {:?}", err);
        }
        Ok(())
    };

    teloxide::repl(bot, handler).await;
}