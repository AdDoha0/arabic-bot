use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use std::env::var;

use super::serializers::{CompletionRequest, CompletionResponse};

#[derive(Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

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
    users_history: HashMap<i64, History>, // HashMap для хранения истории по chat_id
}

impl HistoryCache {
    pub fn new() -> Self {
        Self {
            users_history: HashMap::new(),
        }
    }

    pub fn get_or_create_history(&mut self, chat_id: i64) -> &mut History {
        self.users_history
            .entry(chat_id)
            .or_insert_with(History::new) // Если нет истории, создаём новую
    }

    pub fn get_history(&self, chat_id: i64) -> Option<&History> {
        self.users_history.get(&chat_id)
    }
}

pub trait GetResultApiAi {
    fn get_history_cache(&mut self) -> &mut HistoryCache;

    fn trim_history(messages: &mut Vec<Message>) {
        // Выделяем системные сообщения (контекст)
        let system_messages: Vec<Message> = messages
            .iter()
            .filter(|m| m.role == "system")
            .cloned()
            .collect();

        // Сообщения пользователя и ассистента
        let mut user_assistant_pairs = Vec::new();
        let mut i = 0;

        // Группируем сообщения в пары: запрос пользователя + ответ ассистента
        while i < messages.len() - 1 {
            if i + 1 < messages.len() &&
               messages[i].role == "user" &&
               messages[i+1].role == "assistant" {
                user_assistant_pairs.push((
                    messages[i].clone(),
                    messages[i+1].clone()
                ));
                i += 2;
            } else {
                i += 1;
            }
        }

        // Оставляем только последние 3 пары сообщений
        if user_assistant_pairs.len() > 3 {
            // Удаляем самые старые пары, оставляя только 3 последние
            user_assistant_pairs = user_assistant_pairs.split_off(user_assistant_pairs.len() - 3);
        }

        // Восстанавливаем список сообщений
        let mut result = system_messages;
        for (user_msg, assistant_msg) in user_assistant_pairs {
            result.push(user_msg);
            result.push(assistant_msg);
        }

        *messages = result;
    }


    async fn get_ai_completion(
        &mut self,
        chat_id: i64,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let context = "Тренировка".to_string();
        let request = self.create_completion_request(chat_id, prompt, &context).await?;
        let response = self.send_request(request).await?;

        let reply = match response.choices.first() {
            Some(choice) => choice.message.content.clone(),
            None => "API вернул пустой ответ".to_string(),
        };

        self.get_history_cache()
            .get_or_create_history(chat_id)
            .add_message(Message {
                role: "assistant".to_string(),
                content: reply.clone(),
            });

        Ok(reply)
    }

    async fn create_completion_request(
        &mut self,
        chat_id: i64,
        prompt: &str,
        context: &str,
    ) -> Result<CompletionRequest, Box<dyn std::error::Error>> {
        let ai_model = var("AI_MODEL").expect("Не удалось получить модель AI");
        let history = self.get_history_cache().get_or_create_history(chat_id);

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
            messages: history.messages.clone().into_iter().map(|m| super::serializers::Message {
                role: m.role,
                content: m.content,
            }).collect(),
        })
    }

    async fn send_request(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, Box<dyn std::error::Error>> {
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
