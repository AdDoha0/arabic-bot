use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Textbook {
    pub id: i32,
    pub title: String,
    pub description: String
}
