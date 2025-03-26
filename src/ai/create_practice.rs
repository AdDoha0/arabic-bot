use super::serializers::{CompletionRequest, CompletionResponse};
use super::gpt_client::{GetResultApiAi, History, HistoryCache};





pub struct CreatePractice {
    pub history: History,
    context: String,
    history_cache: HistoryCache,
}

impl CreatePractice {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            context: "Ты - преподаватель арабского языка. Создавай уникальные практические задания \
                     на основе материала урока. Каждая практика должна быть новой и отличаться от \
                     предыдущих. Используй разные форматы заданий для лучшего усвоения материала.".to_string(),
            history_cache: HistoryCache::new(),
        }
    }

    pub async fn get_more_practice(&mut self, chat_id: i64, lesson_text: &str) -> Result<String, anyhow::Error> {
        let unique_id = format!("{}_{}", chat_id, lesson_text.chars().take(20).collect::<String>());

        let prompt = format!(
            "Урок: {}\n\
            Создай новую практику по этому уроку.\n\
            Практика должна:\n\
            1. Строго соответствовать материалу урока\n\
            2. Отличаться от предыдущих практик\n\
            3. Содержать разнообразные упражнения\n\
            4. Быть понятной и структурированной",
            lesson_text
        );

        self.get_ai_completion(chat_id, &prompt, &unique_id).await
    }
}

impl GetResultApiAi for CreatePractice {
    fn get_history_cache(&mut self) -> &mut HistoryCache {
        &mut self.history_cache
    }
}