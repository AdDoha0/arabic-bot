
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

}

pub trait GetResultApiAi {

    fn get_history(&mut self) -> &mut History;

    async fn get_ai_completion(&mut self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;

    async fn create_completion_request(&mut self, prompt: &str, context: &str) -> Result<CompletionRequest, Box<dyn std::error::Error>> {

        let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");
        let history = self.get_history();

        let mut request = CompletionRequest {
            model: ai_model,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: context.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
        };

        if history.messages.len() > 0 {
            for message in history.messages.iter() {
                request.messages.push(message.clone());
            }
        };

        Ok(request)
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

        let response_data = response
            .json::<CompletionResponse>()
            .await?;

        Ok(response_data)
    }
}