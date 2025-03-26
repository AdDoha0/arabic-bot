use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use std::env::var;

use super::serializers::{CompletionRequest, CompletionResponse};
use super::serializers::Message;

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

pub struct HistoryCache {
    users_history: HashMap<String, History>,
}

impl HistoryCache {
    pub fn new() -> Self {
        Self {
            users_history: HashMap::new(),
        }
    }

    pub fn get_or_create_history(&mut self, key: String) -> &mut History {
        self.users_history
            .entry(key)
            .or_insert_with(History::new)
    }

    pub fn get_history(&self, key: String) -> Option<&History> {
        self.users_history.get(&key)
    }
}

pub trait GetResultApiAi {
    fn get_history_cache(&mut self) -> &mut HistoryCache;

    fn trim_history(messages: &mut Vec<Message>) {
        let system_messages: Vec<Message> = messages
            .iter()
            .filter(|m| m.role == "system")
            .cloned()
            .collect();

        let mut user_assistant_msgs: Vec<Message> = messages
            .iter()
            .filter(|m| m.role != "system")
            .cloned()
            .collect();

        while user_assistant_msgs.len() > 4 {
            user_assistant_msgs.drain(0..2);
        }

        *messages = [system_messages, user_assistant_msgs].concat();
    }

    async fn get_ai_completion(
        &mut self,
        chat_id: i64,
        prompt: &str,
        context: &str,
    ) -> Result<String, anyhow::Error> {
        let unique_key = format!("{}_{}", chat_id, context.chars().take(20).collect::<String>());

        let request = self.create_completion_request(&unique_key, prompt, context).await?;
        let response = self.send_request(request).await?;

        let reply = match response.choices.first() {
            Some(choice) => choice.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        self.get_history_cache()
            .get_or_create_history(unique_key)
            .add_message(Message {
                role: "assistant".to_string(),
                content: reply.clone(),
            });

        Ok(reply)
    }

    async fn create_completion_request(
        &mut self,
        key: &str,
        prompt: &str,
        context: &str,
    ) -> Result<CompletionRequest, anyhow::Error> {
        let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");
        let history = self.get_history_cache().get_or_create_history(key.to_string());

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

    async fn send_request(&self, request: CompletionRequest) -> Result<CompletionResponse, anyhow::Error> {
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