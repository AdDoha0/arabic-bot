use teloxide::{prelude::*, utils::command::BotCommands};

use crate::ai::{create_practice::CreatePractice, gpt_client::GetResultApiAi};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Bot info")]
    Help,
    #[command(description = "start the bot")]
    Start,
    #[command(description = "Test function chat gpt")]
    Test,
}


pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let text = "Это команда помощи".to_string();
            bot.send_message(msg.chat.id, text).await?;
        }
        Command::Start => {
            let text  = "Привет! Бот запущен".to_string();
            bot.send_message(msg.chat.id, text).await?;
        }
        Command::Test => {
            let mut practice = CreatePractice::new();

            let text = match practice.get_ai_completion("Я мужчина 30 лет и я люблю ванильное мороженое").await {
                Ok(text) => text,
                Err(_) => "Ошибка при получении ответа от AI".to_string()
            };

            // Вывод истории в консоль после первого запроса
            println!("История после get_ai_completion:");
            for (i, msg) in practice.history.messages.iter().enumerate() {
                println!("{}: [{}] {}", i, msg.role, msg.content);
            }

            // Второй вызов AI
            let text2 = match practice.get_more_practice().await {
                Ok(t) => t,
                Err(_) => "Ошибка при получении ответа от AI".to_string()
            };


            println!("История после get_more_practice:");
            for (i, msg) in practice.history.messages.iter().enumerate() {
                println!("{}: [{}] {}", i, msg.role, msg.content);
            }

            // Отправляем оба ответа в чат
            bot.send_message(msg.chat.id, text).await?;
            bot.send_message(msg.chat.id, text2).await?;
        }

    }
    Ok(())
}




// Обработчик для броска костей
// pub async fn dice_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
//     let practice = CreatePractice::new();
//     practice
//     Ok(())
// }
