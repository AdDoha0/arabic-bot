use async_trait::async_trait;

use crate::ai::gpt_client::GptClient;

pub struct CreatePractice;

#[async_trait]
impl GptClient for CreatePractice {}
