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
    LessonPractice,
    Textbooks,
    // Test,
    Unknown,
    ChoisingVolume
}

impl CallbackType {
    pub fn from_data(data: &str) -> Self {
        if data.starts_with("meeting") {
            Self::Meeting
        } else if data.starts_with("choising_volume"){
            Self::ChoisingVolume
        } else if data.starts_with("volume") {
            Self::Volume
        }
        else if data == "lesson_practice" {
            Self::LessonPractice
        }
        else if data.starts_with("lesson") {
            Self::Lesson
        } else if data.starts_with("practice") {
            Self::Practice
        }else if data.starts_with("pdf") {
            Self::Textbooks
        } else {
            Self::Unknown
        }
    }
}

pub async fn handle_callback_query(bot: Bot, query: CallbackQuery) ->  ResponseResult<()>  {
    let data = query.data.clone().unwrap_or_default();
    bot.answer_callback_query(query.id.clone()).await?;

    match CallbackType::from_data(&data) {
        CallbackType::Meeting => handle_callback_meeting(bot, query).await?,
        CallbackType::Volume => handle_callback_volume(bot, query).await?,
        CallbackType::Lesson => handle_callback_lesson(bot, query).await?,
        CallbackType::Practice => handle_callback_practice(bot, query).await?,
        CallbackType::LessonPractice => handle_callback_lesson_practice(bot, query).await?,
        CallbackType::Textbooks => handle_callback_textbooks_pdf(bot, query).await?,
        CallbackType::ChoisingVolume => handle_callback_сhoosing_volume(bot, query).await?,
        CallbackType::Unknown => {}
    }
    
    Ok(())
}