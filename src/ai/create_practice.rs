
use super::serializers::{CompletionRequest, Message, CompletionResponse};
use super::gpt_client::{GetResultApiAi, History};

// use reqwest::Client;
// use std::env::var;



pub struct CreatePractice {
    pub history: History,
    context: String,

}

impl GetResultApiAi for CreatePractice {

    fn get_history(&mut self) -> &mut History {
        &mut self.history
    }

    async fn get_ai_completion(&mut self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {

        let context = self.context.clone();

        let request = self.create_completion_request(prompt, &context).await?;
        let response = self.send_request(request).await?;

        let reply = match response.choices.first() {
            Some(c) => c.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        Ok(reply)
    }
}


impl CreatePractice {

    pub async fn get_more_practice(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let fixed_prompt = "еще задания. НЕ ПИШИ НИЧЕГО ДРУГОГО КРОМЕ ЗАДАНИЙ. не нужно в начале писать клише, и не пиши вывод";

        let context = self.context.clone();

        let request = self.create_completion_request(fixed_prompt, &context).await?;
        let response = self.send_request(request).await?;

        let reply = match response.choices.first() {
            Some(c) => c.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        let history = self.get_history();
        history.add_message(Message {
            role: "system".to_string(),
            content: reply.clone(),
        });

        // сохраняются только последние 4 сообщения
        if history.messages.len() >= 4 {
            history.messages.remove(0);
        }


        Ok(reply)
    }

}


impl CreatePractice {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            context: "Тренировка".to_string(),
        }
    }
}