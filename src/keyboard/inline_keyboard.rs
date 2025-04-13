use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::types::ReplyMarkup;
use reqwest;
use std::env;

use crate::serializers::{Textbook, LessonId};


pub fn create_inline_keyboard_meeting_button() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Начать!", "meeting_button"),
        ],
    ])
}







pub async fn create_inline_keyboard_сhoosing_volume() -> InlineKeyboardMarkup{

    let mut url = env::var("BECKEND_URL").expect("BECKEND_URL is not set");
    url.push_str("/textbooks");


    let client = reqwest::Client::new();
    let textbooks: Vec<Textbook> = match client
        .get(url)
        .send()
        .await {
            Ok(response) => response.json().await.unwrap_or_default(),
            Err(_) => Vec::new(),
        };

    let mut keyboard = Vec::new();

    for textbook in textbooks {
        let button = InlineKeyboardButton::callback(
            &textbook.title,
            format!("volume_{}", textbook.id)
        );
        keyboard.push(vec![button]);
    }

    // Добавляем кнопку практики в конец
    keyboard.push(vec![InlineKeyboardButton::callback(
        "Практика",
        "practice_ai_communication"
    )]);

    InlineKeyboardMarkup::new(keyboard)
}




pub async fn create_inline_keyboard_сhoosing_lesson(textbook_id: u32) -> InlineKeyboardMarkup {
    let mut url = env::var("BECKEND_URL").expect("BECKEND_URL is not set");
    url.push_str(&format!("/textbooks/{textbook_id}/lessons"));

    let client = reqwest::Client::new();
    let lessons: Vec<LessonId> = match client
       .get(url)
       .send()
       .await {
           Ok(response) => response.json().await.unwrap_or_default(),
           Err(_) => Vec::new(),
       };

    let mut keyboard = Vec::new();

    for lesson in lessons {
        let button = InlineKeyboardButton::callback(
            &lesson.title,
            format!("lesson_{}", lesson.id)
        );
        keyboard.push(vec![button]);
    }

    InlineKeyboardMarkup::new(keyboard)

}









pub fn create_inline_keyboard_сhoosing_ai_assistent() -> InlineKeyboardMarkup {
    let button1 = InlineKeyboardButton::callback("Легко", "practice_state_easy");
    let button2 = InlineKeyboardButton::callback("Нормально", "practice_state_medium");
    let button3 = InlineKeyboardButton::callback("Сложно", "practice_state_hard");

    let row1 = vec![button1];
    let row2 = vec![button2];
    let row3 = vec![button3];

    InlineKeyboardMarkup::new(vec![row1, row2, row3])
}




pub fn create_inline_keyboar_lesson_practice() -> InlineKeyboardMarkup {
    let button1 = InlineKeyboardButton::callback("Сгенирировать задания 🤖", "lesson_practice");

    let row1 = vec![button1];

    InlineKeyboardMarkup::new(vec![row1])

}




