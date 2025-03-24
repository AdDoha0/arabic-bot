use super::serializers::{CompletionRequest, CompletionResponse};
use super::gpt_client::{GetResultApiAi, History, Message, HistoryCache};
use std::env::var;

pub struct CreatePractice {
    pub history: History,
    context: String,
    history_cache: HistoryCache,
}

impl CreatePractice {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            context: "Тренировка".to_string(),
            history_cache: HistoryCache::new(),
        }
    }

    pub async fn get_more_practice(&mut self, lesson_text: &str) -> Result<String, anyhow::Error> {
        let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");

        // Формируем запрос, явно указывая не повторять предыдущие практики
        let prompt = format!(
            "Создай новую практику по уроку: {}. \
            Эта практика должна отличаться от предыдущих практик в истории. \
            Сделай упражнения разнообразными и интересными.",
            lesson_text
        );

        // Проверяем, добавлено ли системное сообщение
        if self.history.messages.iter().all(|m| m.role != "system") {
            self.history.add_message(Message {
                role: "system".to_string(),
                content: self.context.clone(),
            });
        }

        // Добавляем новый запрос от пользователя
        self.history.add_message(Message {
            role: "user".to_string(),
            content: prompt.clone(),
        });

        Self::trim_history(&mut self.history.messages);

        // Отправка запроса в API
        let request = CompletionRequest {
            model: ai_model,
            messages: self.history.messages.clone().into_iter().map(|m| super::serializers::Message {
                role: m.role,
                content: m.content,
            }).collect(),
        };

        let response = self.send_request(request).await?;
        let reply = match response.choices.first() {
            Some(c) => c.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        // Добавляем ответ ассистента в историю
        self.history.add_message(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(reply)
    }
}

impl GetResultApiAi for CreatePractice {
    fn get_history_cache(&mut self) -> &mut HistoryCache {
        &mut self.history_cache
    }
}
