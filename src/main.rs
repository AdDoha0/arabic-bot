use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;
use std::env;


// Импортируем обработчики
mod ai;
mod handlers;
mod keyboard;
use handlers::*;

#[tokio::main]
async fn main() {

    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN not found"));

    // Используем обработчики из модуля handlers
    let handler: Handler<'_, DependencyMap, Result<(), teloxide::RequestError>, teloxide::dispatching::DpHandlerDescription> = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(command_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
