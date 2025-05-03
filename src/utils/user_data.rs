use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;


#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub lesson_id: String,
    pub lesson_text: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserDataCache {
    users: HashMap<i64, UserData>,
    dirty: bool,
    file_path: String,
}


static CACHE: Lazy<Mutex<UserDataCache>> = Lazy::new(|| {
    let cache = UserDataCache::load_from_file("user_lessons.json")
    .unwrap_or_else(|_| UserDataCache::default());
    Mutex::new(cache)
});


impl UserDataCache {
    pub fn new(file_path: &str) -> Self {
        Self {
            users: HashMap::new(),
            dirty: false,
            file_path: file_path.to_string(),
        }
    }

    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        if !Path::new(file_path).exists() {
            return Ok(Self::new(file_path));
        }

        let content = fs::read_to_string(file_path)?;
        let mut cache: Self = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        cache.file_path = file_path.to_string();
        Ok(cache)
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.file_path, json)
    }


    pub fn update_user_lesson(&mut self, chat_id: i64, lesson_id: String, lesson_text: String) {
        self.users.insert(chat_id, UserData { lesson_id, lesson_text });
        self.dirty = true;

        // Периодически сохраняем на диск
        if self.users.len() % 10 == 0 {
            if let Err(e) = self.save_to_file() {
                eprintln!("Ошибка при сохранении кэша: {}", e);
            } else {
                self.dirty = false;
            }
        }
    }

    pub fn get_user_lesson_text(&self, chat_id: i64) -> Option<String> {
        self.users.get(&chat_id).map(|user_data| user_data.lesson_text.clone())
    }

    pub fn get_user_lesson_id(&self, chat_id: i64) -> Option<String> {
        self.users.get(&chat_id).map(|user_data| user_data.lesson_id.clone())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.dirty {
            self.save_to_file()?;
            self.dirty = false;
        }
        Ok(())
    }
}



pub fn save_user_lesson(
    chat_id: i64,
    lesson_id: String,
    lesson_text: String
) -> io::Result<()> {
    let mut cache = CACHE.lock().unwrap();
    cache.update_user_lesson(chat_id, lesson_id, lesson_text);
    Ok(())
}

pub fn get_user_lesson_text(chat_id: i64) -> Option<String> {
    let cache = CACHE.lock().unwrap();
    cache.get_user_lesson_text(chat_id)
}

pub fn flush_cache() -> io::Result<()> {
    let mut cache = CACHE.lock().unwrap();
    cache.flush()
}
