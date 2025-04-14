use anyhow::{Result, Context}; // Для работы с anyhow


pub async fn load_context() -> Result<String> {
    tokio::fs::read_to_string("src/assets/context.txt")
        .await
        .context("Не удалось загрузить контекст из файла") // Добавляем контекст для ошибок
}