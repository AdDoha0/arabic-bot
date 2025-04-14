use teloxide::{
    prelude::*,
    types::{CallbackQuery, Message}
};
use reqwest;
use std::env;
use teloxide::RequestError;
use std::io;

use crate::{ai::gpt_client::GptClient, keyboard::inline_keyboard::*};
use crate::utils::user_data::{save_user_lesson};
use crate::serializers::{Lesson};
use crate::utils::load_context;


use crate::ai::create_practice::CreatePractice;
use crate::utils::user_data::get_user_lesson_text;


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


pub async fn handle_callback_lesson_practice(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {
        // Отправляем сообщение "Пожалуйста, подождите..." и сохраняем его чтоб удалить его позже
        let waiting_message: Message = bot
            .send_message(message.chat().id, "Пожалуйста, подождите...")
            .await?;

        let context = load_context()
            .await
            .map_err(|e| RequestError::from(io::Error::new(io::ErrorKind::Other, e)))?;

        let lesson_text = match get_user_lesson_text(message.chat().id.0) {
            Some(text) => text,
            None => {
                log::warn!("У пользователя нет выбранного урока");
                return Ok(());
            }
        };

        let client = CreatePractice {};
        let response = client
            .get_completion(&lesson_text, &context)
            .await
            .map_err(|e| RequestError::from(io::Error::new(io::ErrorKind::Other, e)))?;

        log::info!("Отправлен урок  API");


         // Удаляем сообщение "Пожалуйста, подождите..."
         bot.delete_message(waiting_message.chat.id, waiting_message.id)
            .await?;


        bot.send_message(message.chat().id, response)
           .await?;

    }

    Ok(())
}

