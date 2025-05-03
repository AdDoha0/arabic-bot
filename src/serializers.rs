use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Textbook {
    pub id: i32,
    pub title: String,
    pub description: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LessonId {
    pub id: i32,
    pub title: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub video_url: Option<String>,
    pub textbook_id: i32,
    pub created_at: String,
}



impl Default for Lesson {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            text: String::new(),
            video_url: None,
            textbook_id: 0,
            created_at: String::new(),
        }
    }
}