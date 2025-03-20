use teloxide::{
    prelude::*,
    types::{CallbackQuery, Message},
    utils::command::BotCommands,
};

use crate::keyboard::inline_keyboard::*;

// Обработчик для кнопки начала обучения
pub async fn handle_callback_meeting(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {
        log::info!("Создаем клавиатуру выбора тома");
        let keyboard = create_inline_keyboard_сhoosing_volume();
        log::info!("Отправляем сообщение с клавиатурой");
        bot.send_message(message.chat().id, "Выберите действие:")
            .reply_markup(keyboard)
            .await?;
        log::info!("Сообщение успешно отправлено");
    } else {
        log::warn!("В колбэке отсутствует сообщение");
    }

    Ok(())
}

// Обработчик для выбора тома
pub async fn handle_callback_volume(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {
        let keyboard = create_inline_keyboard_сhoosing_lesson();
        bot.send_message(message.chat().id, "Выберите урок для тома")
            .reply_markup(keyboard)
            .await?;
        }


    Ok(())
}

// Обработчик для выбора урока
pub async fn handle_callback_lesson(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {
        bot.send_message(message.chat().id, "Вы выбрали урок")
            .await?;
    }

    Ok(())
}


// Обработчик для выбора практики
pub async fn handle_callback_practice(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {
        // В будущем тут можно добавить логику для разных типов практики
        bot.send_message(message.chat().id, format!("Вы выбрали практику"))
            .await?;
    }

    Ok(())
}



