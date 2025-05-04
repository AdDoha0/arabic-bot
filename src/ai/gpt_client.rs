
use reqwest::Client;

use super::serializers::{CompletionRequest, CompletionResponse};
use super::serializers::Message;


use anyhow::Result;
use async_trait::async_trait;

use std::env;

#[async_trait]
pub trait GptClient {
    async fn build_request(&self, prompt: &str, context: &str) -> CompletionRequest {
        let model = env::var("AI_MODEL").expect("AI_MODEL not set");
        
        CompletionRequest {
            model,
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
        }
    }

    async fn send_request(&self, request: CompletionRequest) -> Result<CompletionResponse, anyhow::Error> {
        let api_key = env::var("AI_API_KEY")?;
        let base_url = env::var("BASE_URL")?;

        let client = Client::new();
        let response = client
            .post(base_url)
            .bearer_auth(api_key)
            .json(&request)
            .send()
            .await?;

         // Получаем тело ответа как строку
        let text = response.text().await?;

        // Логируем необработанный JSON
        log::info!("AI raw response: {}", text);

        // Парсим строку в структуру
        let result: CompletionResponse = serde_json::from_str(&text)?;
        Ok(result)

        // let result = response.json::<CompletionResponse>().await?;
        // Ok(result)
    }

    async fn get_completion(&self, prompt: &str, context: &str) -> Result<String, anyhow::Error> {
        let req = self.build_request(prompt, context).await;
        let res = self.send_request(req).await?;

        Ok(res
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "Нет ответа от модели".to_string()))
    }
}





