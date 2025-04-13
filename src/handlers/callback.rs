use teloxide::{
    RequestError,
    prelude::*,
    types::{CallbackQuery, Message},
    utils::{self, command::BotCommands},
};
use reqwest;
use std::env;


use crate::keyboard::inline_keyboard::*;
use crate::utils::user_data::{save_user_lesson, get_user_lesson_text};
use crate::utils::auxiliary_fn::escape_markdown;
// use crate::ai::CreatePractice;
use crate::serializers::{Lesson};


// Обработчик для кнопки начала обучения
pub async fn handle_callback_meeting(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

    if let Some(message) = query.message {
        let keyboard = create_inline_keyboard_сhoosing_volume().await;
        bot.send_message(message.chat().id, "Выберите действие:")
            .reply_markup(keyboard)
            .await?;
    } else {
        log::warn!("В колбэке отсутствует сообщение");
    }

    Ok(())
}

// Обработчик для выбора тома
pub async fn handle_callback_volume(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {


    let textbook_id = query.data
            .as_ref()
            .map(|data| {
                log::info!("Получен callback_data: {}", data);
                data
            })
            .and_then(|data| data.strip_prefix("volume_"))
            .and_then(|id| id.parse::<u32>().ok())
            .unwrap_or(1);

    if let Some(message) = query.message {
        let keyboard = create_inline_keyboard_сhoosing_lesson(textbook_id).await;
            bot.send_message(message.chat().id, "Выберите урок для тома")
                .reply_markup(keyboard)
                .await?;
    }
    Ok(())
}




// Обработчик для выбора урока
pub async fn handle_callback_lesson(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

    let lesson_id = query.data
            .as_ref()
            .map(|data| {
                log::info!("Получен callback_data: {}", data);
                data
            })
            .and_then(|data| data.strip_prefix("lesson_"))
            .and_then(|id| id.parse::<u32>().ok())
            .unwrap_or(1);


    let client = reqwest::Client::new();
    let lesson: Lesson = match client
       .get(format!("{}/lessons/{}", env::var("BECKEND_URL").unwrap(), lesson_id))
       .send()
       .await {
        Ok(response) => response.json().await.unwrap(),
        Err(e) => {
            log::error!("Ошибка при запросе урока: {e}");
            Lesson::default()
        }
    };

    if let Some(message) = query.message {
        let chat_id = message.chat().id.0;

        // Сохраняем ID урока и его текст
        if let Some(data) = &query.data {
            if let Err(e) = save_user_lesson(chat_id, data.clone(), lesson.text.clone()) {
                log::warn!("Ошибка при сохранении урока: {}", e);
            }
        }

        bot.send_message(message.chat().id, lesson.text)
            // .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        let practice_keyboard = create_inline_keyboar_lesson_practice();
        bot.send_message(message.chat().id, "Хотите практику по этому уроку?")
            .reply_markup(practice_keyboard)
            .await?;
    }

    Ok(())
}



pub async fn handle_callback_practice(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

    if let Some(message) = query.message {
        // В будущем тут можно добавить логику для разных типов практики
        bot.send_message(message.chat().id, format!("Вы выбрали практику"))
            .await?;
    }

    Ok(())
}





// pub async fn handle_callback_lesson_practice(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

//     log::info!("Начало обработки колбэка 'handle_callback_lesson_practice");

//     if let Some(message) = query.message {

//         let chat_id = message.chat().id.0;


//         match get_user_lesson_text(chat_id) {
//             Some(lesson_text) => {
//                 // Здесь можно использовать lesson_text (последний урок пользователя)

//                 log::info!("Текст урока | последний урок открытый пользователем: {}", lesson_text);

//                 // let escaped_lesson_text = escape_markdown(&lesson_text);

//                 bot.send_message(message.chat().id,
//                     "Генерирую практику на основе изученного материала...").await?;


//                 let mut practie = CreatePractice::new();

//                 match practie.get_more_practice(chat_id, &lesson_text).await {
//                     Ok(practice) => {
//                         // let escaped_practice = escape_markdown(&practice);
//                         // log::info!("экранирование спец символов прошло успешно");

//                         log::info!("Отправляю практику пользователю");
//                         log::info!("Практика: {}", practice);

//                         bot.send_message(message.chat().id, practice)
//                            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
//                            .await?;
//                         log::info!("Практика успешно сгенерирована и отправлена пользователю");
//                     }
//                     Err(e) => {
//                         log::error!("Ошибка при генерации практики: {}", e);
//                         bot.send_message(message.chat().id,
//                             "Извините, произошла ошибка при генерации практики. Попробуйте позже.")
//                            .await?;
//                     }
//                 }
//             },
//             None => {
//                 bot.send_message(message.chat().id,
//                     "Не могу найти текст урока. Пожалуйста, выберите урок снова.")
//                     .await?;
//             }
//         }
//     }
//     Ok(())
// }
