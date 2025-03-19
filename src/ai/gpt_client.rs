use super::serializers::{CompletionRequest, Message, CompletionResponse,};
use reqwest::Client;
use std::env::var;

// Request — это сообщение, которое клиент отправляет серверу
// Response — это сообщение, которое сервер отправляет клиенту в ответ на запрос.

#[derive(Clone)]
pub struct History {
    pub messages: Vec<Message>,
}

impl History {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn print_history(&self) {
        println!("---------------- История сообщений ----------------");
        for (i, msg) in self.messages.iter().enumerate() {
            println!("{}: [{}] {}", i, msg.role, msg.content);
        }
        println!("--------------------------------------------------\n");
    }
}


pub trait GetResultApiAi {
    fn get_history(&mut self) -> &mut History;

    fn trim_history(messages: &mut Vec<Message>) {
        // Отделим системное сообщение
        let system_messages: Vec<Message> = messages
            .iter()
            .filter(|m| m.role == "system")
            .cloned()
            .collect();

        // Соберём все user/assistant
        let mut user_assistant_msgs: Vec<Message> = messages
            .iter()
            .filter(|m| m.role != "system")
            .cloned()
            .collect();

        // Оставим последние 6 сообщений (3 user + 3 assistant)
        while user_assistant_msgs.len() > 6 {
            user_assistant_msgs.drain(0..2); // удаляем самую старую user+assistant пару
        }

        *messages = [system_messages, user_assistant_msgs].concat();
    }

    async fn get_ai_completion(&mut self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let context = "Тренировка".to_string();
        let request = self.create_completion_request(prompt, &context).await?;
        let response = self.send_request(request).await?;

        let reply = match response.choices.first() {
            Some(choice) => choice.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        // Сохраняем ответ от ассистента
        self.get_history().add_message(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(reply)
    }

    async fn create_completion_request(&mut self, prompt: &str, context: &str) -> Result<CompletionRequest, Box<dyn std::error::Error>> {
        let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");
        let history = self.get_history();

        // Добавляем системное сообщение, если история пуста
        if history.messages.iter().all(|m| m.role != "system") {
            history.add_message(Message {
                role: "system".to_string(),
                content: context.to_string(),
            });
        }

        history.add_message(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        Self::trim_history(&mut history.messages);

        Ok(CompletionRequest {
            model: ai_model,
            messages: history.messages.clone(),
        })
    }

    async fn send_request(&self, request: CompletionRequest) -> Result<CompletionResponse, Box<dyn std::error::Error>> {
        let api_key = var("AI_API_KEY").expect("Не удалось получить API ключ");
        let base_url = var("BASE_URL").expect("Не удалось получить URL API");

        let client = Client::new();
        let response = client
            .post(base_url)
            .bearer_auth(api_key)
            .json(&request)
            .send()
            .await?;

        let response_data = response.json::<CompletionResponse>().await?;
        Ok(response_data)
    }
}