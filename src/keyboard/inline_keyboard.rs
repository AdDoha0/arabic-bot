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


pub fn create_inline_keyboard_сhoosing_volume() -> InlineKeyboardMarkup {
    // Упрощаем создание клавиатуры
    let button1 = InlineKeyboardButton::callback("Мединский курс (том 1)", "volume_1");
    let button2 = InlineKeyboardButton::callback("Мединский курс (том 2)", "volume_2");
    let button3 = InlineKeyboardButton::callback("Мединский курс (том 3)", "volume_3");
    let button4 = InlineKeyboardButton::callback("Практика", "practice_ai_communication");

    let row1 = vec![button1];
    let row2 = vec![button2];
    let row3 = vec![button3];
    let row4 = vec![button4];

    let keyboard = vec![row1, row2, row3, row4];

    InlineKeyboardMarkup::new(keyboard)
}


pub fn create_inline_keyboard_сhoosing_lesson() -> InlineKeyboardMarkup {
    let button1 = InlineKeyboardButton::callback("Урок 1", "lesson_1");
    let button2 = InlineKeyboardButton::callback("Урок 2", "lesson_2");
    let button3 = InlineKeyboardButton::callback("Урок 3", "lesson_3");

    let row1 = vec![button1];
    let row2 = vec![button2];
    let row3 = vec![button3];

    InlineKeyboardMarkup::new(vec![row1, row2, row3])
}



pub fn create_inline_keyboard_сhoosing_practice() -> InlineKeyboardMarkup {
    let button1 = InlineKeyboardButton::callback("Легко", "practice_state_easy");
    let button2 = InlineKeyboardButton::callback("Нормально", "practice_state_medium");
    let button3 = InlineKeyboardButton::callback("Сложно", "practice_state_hard");

    let row1 = vec![button1];
    let row2 = vec![button2];
    let row3 = vec![button3];

    InlineKeyboardMarkup::new(vec![row1, row2, row3])
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



