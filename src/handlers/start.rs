use teloxide::{prelude::*, utils::command::BotCommands};

use crate::keyboard::keyboard::create_inline_keyboard_meeting_button;
use crate::ai::{create_practice::CreatePractice, gpt_client::GetResultApiAi};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Bot info")]
    Help,
    #[command(description = "start the bot")]
    Start,
}


pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let text = "Это команда помощи".to_string();
            bot.send_message(msg.chat.id, text).await?;
        }
        Command::Start => {
            let text  = "
                Ассаляму алейкум! 🌙

Ты в боте для изучения арабского языка.
Твоя цель — шаг за шагом погружаться в мир арабского, улучшать разговорные навыки и чувствовать себя уверенно в общении.

✨ Ждут упражнения, диалоги и новые слова.

Готов начать обучение? Пиши /help, чтобы узнать больше или сразу выбери тему для практики.
            ".to_string();
            let keyboard = create_inline_keyboard_meeting_button();

            bot.send_message(msg.chat.id, text)
                .reply_markup(keyboard)
                .await?;
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
