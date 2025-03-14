
use teloxide::{prelude::*, utils::command::BotCommands};


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "throw dice.")]
    Help,
    #[command(description = "start the bot.")]
    Start,
}


pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let text = "Это команда помощи".to_string();
            bot.send_message(msg.chat.id, text).await?;
        }
        Command::Start => {
            let text  = "Привет! Бот запущен.".to_string();
            bot.send_message(msg.chat.id, text).await?;
        }
    }
    Ok(())
}




// // Обработчик для броска костей
// pub async fn dice_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
//     if let Err(err) = bot.send_dice(msg.chat.id).await {
//         log::error!("Error sending dice: {:?}", err);
//     }
//     Ok(())
// }
