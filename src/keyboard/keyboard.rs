
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::types::ReplyMarkup;



pub fn create_inline_keyboard_meeting_button() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Начать!", "meeting_button"),
        ],
    ])
}


pub  fn create_inline_keyboard_сhoosing_textbook() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Мединский курс (том 1)", "volume_1"),
        ],
        vec![
            InlineKeyboardButton::callback("Мединский курс (том 2)", "volume_2"),
        ],
        vec![
            InlineKeyboardButton::callback("Мединский курс (том 3)", "volume_3"),
        ],
        vec![
            InlineKeyboardButton::callback("Практика", "practice_ai_communication"),
        ],
    ])
}


pub fn create_inline_keyboard_сhoosing_lesson() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Урок 1", "lesson_1"),
        ],
        vec![
            InlineKeyboardButton::callback("Урок 2", "lesson_2"),
        ],
        vec![
            InlineKeyboardButton::callback("Урок 3", "lesson_3"),
        ],
    ])
}




// pub fn create_inline_keyboard_start() -> InlineKeyboardMarkup {
//     let mut  keyboard = InlineKeyboardMarkup::new(vec![]);

//     for i in 1..4 {
//         let r = vec![
//             InlineKeyboardButton::callback("ТЕстовая кнопка {i}", "meeting_button"),
//         ];
//         keyboard.inline_keyboard.push(r);
//     };

//     keyboard
// }



