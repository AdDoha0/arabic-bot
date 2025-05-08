use teloxide::{
    prelude::*, types::{CallbackQuery, InputFile, Message}
};
use reqwest;
use std::env;
use teloxide::RequestError;
use std::io;


use crate::{ai::gpt_client::GptClient, keyboard::inline_keyboard::*};
use crate::utils::user_data::save_user_lesson;
use crate::serializers::Lesson;
use crate::utils::auxiliary_fn::{load_context, list_files_in_dir};


use crate::ai::create_practice::CreatePractice;
use crate::utils::user_data::get_user_lesson_text;




pub async fn handle_callback_сhoosing_volume(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

    if let Some(message) = query.message {
        let keyboard = create_inline_keyboard_сhoosing_volume().await;

        bot.send_message(message.chat().id, "📚 Выберите учебник, чтобы получить конспекты")
            .reply_markup(keyboard)        
            .await?;

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
    } else {
        log::warn!("Ошибка в колбеке handle_callback_volume");
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
    let url = format!("{}/lessons/{}", env::var("BECKEND_URL").unwrap(), lesson_id);
    log::info!("Запрос к URL: {}", url);
    
    let lesson: Lesson = match client
       .get(&url)
       .send()
       .await {
        Ok(response) => {
            let text = response.text().await.unwrap_or_default();
            log::info!("Ответ от сервера: {}", text);
            match serde_json::from_str::<Lesson>(&text) {
                Ok(lesson) => lesson,
                Err(e) => {
                    log::error!("Ошибка при десериализации урока: {}", e);
                    log::error!("Текст ответа: {}", text);
                    Lesson::default()
                }
            }
        },
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

        
        if let Some(video) = &lesson.video_url {
            bot.send_message(message.chat().id, video)
                .await?;
        } 

        bot.send_message(message.chat().id, lesson.text)
            // .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        let practice_keyboard = create_inline_keyboar_lesson_practice();
        bot.send_message(message.chat().id, "Хотите практику по этому уроку?")
            .reply_markup(practice_keyboard)
            .await?;
    } else {
        log::warn!("Ошибка в колбеке handle_callback_lesson");
    }



    Ok(())
}



pub async fn handle_callback_practice(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

    if let Some(message) = query.message {
        // В будущем тут можно добавить логику для разных типов практики
        bot.send_message(message.chat().id, format!("Вы выбрали практику"))
            .await?;
    } else {
        log::warn!("Ошибка в колбеке handle_callback_practice");
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

    } else {
        log::warn!("Ошибка в колбеке handle_callback_lesson_practice");
    }

    Ok(())
}


// ------------------------Ui-----------------------------


pub async fn handle_callback_meeting(bot: Bot, query: CallbackQuery) ->  ResponseResult<()> {

    if let Some(message) = query.message {

        let keyboard = create_сinline_keyboar_action();
        
        bot.send_message(message.chat().id, "📚 Материалы для изучения арабского языка:
Ты можешь выбрать, с чего начать:

🔹 Конспекты — краткие, структурированные материалы для быстрого повторения и понимания.
🔹 Книги — полноценные учебники для глубокого и системного изучения языка.

Выбери нужный вариант ниже 👇
        ")
            .reply_markup(keyboard)
            .await?;
    } else {
        log::warn!("Ошибка в коллбеке handle_callback_meeting");
    }

    Ok(())
}



pub async fn handle_callback_textbooks_pdf(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {

        bot.send_message(message.chat().id, "Вот подборка учебников по арабскому языку
Они подойдут как для начинающих, так и для тех, кто уже делает успехи. Изучай в удобном темпе — и пусть Аллах облегчит тебе путь к знанию! 🤲
t.me/test24354 👈 тыкай сюда").await?;
        
    } 
    Ok(())
}