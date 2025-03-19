use super::serializers::{CompletionRequest, Message, CompletionResponse};
use super::gpt_client::{GetResultApiAi, History};
use std::env::var;

pub struct CreatePractice {
    pub history: History,
    context: String,
}

impl CreatePractice {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            context: "Тренировка".to_string(),
        }
    }

    pub async fn get_more_practice(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");
        // let fixed_prompt = "Сделай новую практику по теме, избегая повторов заданий выше.".to_string();
        let fixed_prompt = "Расскажи про МЕНЯ".to_string();

        // Добавляем новый запрос от пользователя
        {
            let history = self.get_history();
            history.add_message(Message {
                role: "user".to_string(),
                content: fixed_prompt.clone(),
            });

            Self::trim_history(&mut history.messages);
        }

        let request = CompletionRequest {
            model: ai_model,
            messages: self.get_history().messages.clone(),
        };

        let response = self.send_request(request).await?;
        let reply = match response.choices.first() {
            Some(c) => c.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        self.get_history().add_message(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(reply)
    }
}

impl GetResultApiAi for CreatePractice {
    fn get_history(&mut self) -> &mut History {
        &mut self.history
    }
}
