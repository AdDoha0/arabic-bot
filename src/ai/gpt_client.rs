
use super::serializers::{CompletionRequest, Message, CompletionResponse,};
use reqwest::Client;
use std::env::var;

// Request — это сообщение, которое клиент отправляет серверу
// Response — это сообщение, которое сервер отправляет клиенту в ответ на запрос.

pub async fn get_ai_completion(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {

    let api_key = var("AI_API_KEY").expect("Не удалось получить API ключ");
    let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");
    let base_url = var("BASE_URL").expect("Не удалось получить URL API");


    let client = Client::new();

    let request = CompletionRequest {
        model: ai_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(), // Добавляем системное сообщение
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }
        ],
    };

    let response = client
        .post(base_url)
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await?;


    // let response_text = response.text().await?;
    // println!("Ответ API: {}", response_text);

    let response_data = response
        .json::<CompletionResponse>()
        .await?;

    let reply = match response_data.choices.first() {
        Some(c) => c.message.content.clone(),
        None => "Нет ответа".to_string(),
    };

    Ok(reply)

}
