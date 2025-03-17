use serde::{Deserialize, Serialize};

// {
//     "model": "CohereForAI/c4ai-command-r-plus-08-2024",
//     "messages": [
//         {
//             "role": "system",
//             "content": "You are a helpful assistant."
//         },
//         {
//             "role": "user",
//             "content": "Hello!"
//         }
//     ]
// }

// ---------------Request-----------------------
#[derive(Serialize, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// ---------------Response-----------------------

#[derive(Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Deserialize, Debug)]
pub struct MessageContent {
    pub content: String,
}

