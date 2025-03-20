use teloxide::prelude::*;
use teloxide::types::CallbackQuery;
use teloxide::RequestError;

use super::callback::*;
use crate::keyboard::inline_keyboard::*;

pub enum CallbackType {
    Meeting,
    Volume,
    Lesson,
    Practice,
    Unknown,
}

impl CallbackType {
    fn from_data(data: &str) -> Self {
        if data.starts_with("meeting") {
            Self::Meeting
        } else if data.starts_with("volume") {
            Self::Volume
        } else if data.starts_with("lesson") {
            Self::Lesson
        } else if data.starts_with("practice") {
            Self::Practice
        } else {
            Self::Unknown
        }
    }
}

pub async fn handle_callback_query(bot: Bot, query: CallbackQuery) -> Result<(), RequestError> {
    let data = query.data.clone().unwrap_or_else(|| String::from(""));
    log::info!("Получен колбэк с данными: {}", data);

    // Сразу отвечаем на callback query, чтобы убрать анимацию загрузки у кнопки
    bot.answer_callback_query(query.id.clone()).await?;
    log::info!("Отправлен ответ на callback_query для снятия анимации загрузки");

    let callback_type = CallbackType::from_data(&data);
    log::info!("Определен тип колбэка: {:?}", match callback_type {
        CallbackType::Meeting => "Meeting",
        CallbackType::Volume => "Volume",
        CallbackType::Lesson => "Lesson",
        CallbackType::Practice => "Practice",
        CallbackType::Unknown => "Unknown",
    });

    match callback_type {
        CallbackType::Meeting => {
            log::info!("Вызываем handle_callback_meeting");
            if let Err(err) = handle_callback_meeting(bot, query).await {
                log::error!("Ошибка в handle_callback_meeting: {:?}", err);
                return Err(err);
            }
        },
        CallbackType::Volume => {
            log::info!("Вызываем handle_callback_volume");
            if let Err(err) = handle_callback_volume(bot, query).await {
                log::error!("Ошибка в handle_callback_volume: {:?}", err);
                return Err(err);
            }
        },
        CallbackType::Lesson => {
            log::info!("Вызываем handle_callback_lesson");
            if let Err(err) = handle_callback_lesson(bot, query).await {
                log::error!("Ошибка в handle_callback_lesson: {:?}", err);
                return Err(err);
            }
        },
        CallbackType::Practice => {
            log::info!("Вызываем handle_callback_practice");
            if let Err(err) = handle_callback_practice(bot, query).await {
                log::error!("Ошибка в handle_callback_practice: {:?}", err);
                return Err(err);
            }
        },
        CallbackType::Unknown => {
            log::warn!("Неизвестный callback data: {}", data);
        }
    }

    log::info!("Обработка колбэка завершена успешно");
    Ok(())
}