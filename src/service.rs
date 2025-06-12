use amico_core::types::ChatMessage;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Aimo AI API model.
///
/// TODO: Integrate amico_sdk's `Model` trait.
///
/// Waiting for amico Model to support WASM.
#[derive(Debug)]
pub struct AimoModel {
    base_url: String,
    jwt: String,
    client: Client,
}

const AIMO_BASE_URL: &str = "https://ai.aimoverse.xyz/api/v1.0.0";

impl AimoModel {
    /// Create a new AimoModel.
    pub fn new(jwt: String) -> Self {
        let client = Client::new();
        Self {
            jwt,
            client,
            base_url: AIMO_BASE_URL.to_string(),
        }
    }

    /// Send a completion request to the Aimo model.
    pub async fn completion(&self, messages: &Vec<ChatMessage>) -> anyhow::Result<String> {
        let request = RequestSchema {
            model: "aimo-chat".to_string(),
            messages: messages.clone(),
            temperature: 0.5,
            max_tokens: 1000,
            top_p: 0.95,
            stream: 0,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.jwt))
            .json(&request)
            .send()
            .await?
            .json::<ResponseSchema>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestSchema {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
    max_tokens: u64,
    top_p: f32,
    stream: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseSchema {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChoiceSchema>,
    usage: UsageSchema,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChoiceSchema {
    index: u32,
    message: ChatMessage,
    finish_reason: String,
    delta: Option<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsageSchema {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
