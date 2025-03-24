use teloxide::{
    prelude::*,
    dispatching::{Dispatcher, UpdateHandler},
    types::{CallbackQuery, Message},
};
use dotenv::dotenv;
use std::env;

mod ai;
mod handlers;
mod keyboard;
mod utils;

use handlers::start::handle_callback_query;

// Удаляем #[tokio::main] чтобы вручную настроить рантайм
fn main() {
    // Инициализация переменных окружения и логгера
    dotenv().ok();

    // Устанавливаем максимальный уровень логирования (trace, debug, info, warn, error)
    std::env::set_var("RUST_LOG", "info,telegram_bot=debug,teloxide=info");
    pretty_env_logger::init();

    log::info!("Запуск бота...");

    // Увеличиваем размер стека для потоков, чтобы избежать ошибки переполнения стека
    let thread_stack_size = 8 * 1024 * 1024; // 8MB
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_stack_size(thread_stack_size)
        .enable_all()
        .build()
        .expect("Не удалось создать рантайм");

    // Запускаем асинхронную функцию внутри рантайма
    rt.block_on(async {
        let bot = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN не найден"));
        log::info!("Бот инициализирован");

        // Основной обработчик через dptree
        log::info!("Настройка обработчиков команд и колбэков");
        let handler = dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<handlers::cmd::Command>()
                    .endpoint(|bot: Bot, msg: Message, cmd: handlers::cmd::Command| {
                        handlers::cmd::command_handler(bot, msg, cmd)
                    })
            )
            .branch(
                Update::filter_callback_query()
                    .endpoint(handle_callback_query)
            );

        log::info!("Запуск диспетчера бота");

        let mut dispatcher = Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build();

        log::info!("Бот запущен и готов обрабатывать сообщения");

        dispatcher.dispatch().await;

        log::info!("Бот остановлен");
    });
}