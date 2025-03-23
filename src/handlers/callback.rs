use teloxide::{
    prelude::*,
    types::{CallbackQuery, Message},
    utils::command::BotCommands,
};

use crate::keyboard::inline_keyboard::*;

// Обработчик для кнопки начала обучения
pub async fn handle_callback_meeting(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

    if let Some(message) = query.message {
        let keyboard = create_inline_keyboard_сhoosing_volume();
        bot.send_message(message.chat().id, "Выберите действие:")
            .reply_markup(keyboard)
            .await?;
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
    struct Lesson {
        text: String,
    }


    let lesson = Lesson {
        text: "📚 *Урок: Идафная конструкция \\(الإضافة\\)*\n\n\
        🔤 *Определение:*\n\
        Идафа \\- это особая грамматическая конструкция в арабском языке, выражающая принадлежность или отношение между двумя существительными\\.\n\n\
        📝 *Правила построения:*\n\
        1\\. Первое слово \\(مضاف\\) теряет определенный артикль الـ и танвин\n\
        2\\. Второе слово \\(مضاف إليه\\) всегда стоит в родительном падеже\n\n\
        🎯 *Примеры:*\n\
        • `كِتَابُ الطَّالِبِ` \\(китабу\\-т\\-талиби\\) \\- книга студента\n\
        `كِتَابُ` \\(книга\\) \\+ `الطَّالِبِ` \\(студента\\)\n\n\
        • `بَيْتُ المُدَرِّسِ` \\(байту\\-ль\\-мударриси\\) \\- дом учителя\n\
        `بَيْتُ` \\(дом\\) \\+ `المُدَرِّسِ` \\(учителя\\)\n\n\
        🔍 *Важные замечания:*\n\
        1\\. Можно строить цепочки: `مُدِيرُ مَدْرَسَةِ المَدِينَةِ` \\(директор школы города\\)\n\
        2\\. Первое слово никогда не имеет артикля\n\
        3\\. Огласовки конца первого слова меняются по правилам إعراب\n\n\
        ⚠️ *Частые ошибки:*\n\
        • Добавление артикля к первому слову\n\
        • Неправильные падежные окончания\n\
        • Разрыв идафной цепочки прилагательным
        ".to_string(),
    };

    if let Some(message) = query.message {
        bot.send_message(message.chat().id, lesson.text)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
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


// pub async fn handle_callback_lesson_practice(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {

//     if let Some(message) = query.message {

//     }


// }
