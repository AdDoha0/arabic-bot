use anyhow::{Result, Context}; // Для работы с anyhow

use std::fs;
use std::path::{Path, PathBuf};


pub async fn load_context() -> Result<String> {
    tokio::fs::read_to_string("src/assets/context.txt")
        .await
        .context("Не удалось загрузить контекст из файла") // Добавляем контекст для ошибок
}



pub fn list_files_in_dir(dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }

    files
}
