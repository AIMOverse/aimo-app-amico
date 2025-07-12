use std::sync::Arc;

use amico_core::{
    Agent,
    types::{Chat, ChatMessage},
};
use tokio::sync::Mutex;
use tokio_with_wasm::alias as tokio;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod agent;
mod log;
mod service;
mod note;

use agent::{AppStrategy, ChatHandler, create_agent};

/// A WASM-bindgen compatible message structure that can be converted to ChatMessage.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Message {
    content: String,
    role: String,
}

#[wasm_bindgen]
impl Message {
    #[wasm_bindgen(constructor)]
    pub fn new(content: String, role: String) -> Message {
        Message { content, role }
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> String {
        self.content.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn role(&self) -> String {
        self.role.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    #[wasm_bindgen(setter)]
    pub fn set_role(&mut self, role: String) {
        self.role = role;
    }
}

impl From<Message> for ChatMessage {
    fn from(message: Message) -> Self {
        ChatMessage {
            content: message.content,
            role: message.role,
        }
    }
}

/// The WASM runtime for the agent.
#[wasm_bindgen]
pub struct AgentWasmRuntime {
    agent: Option<Agent<AppStrategy>>,
    chat_handler: Arc<Mutex<ChatHandler>>,
    running: bool,
}

#[wasm_bindgen]
impl AgentWasmRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new(jwt: String) -> AgentWasmRuntime {
        let (agent, chat_handler) = create_agent(jwt);

        AgentWasmRuntime {
            agent: Some(agent),
            chat_handler: Arc::new(Mutex::new(chat_handler)),
            running: false,
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        if self.running {
            tracing::warn!("Agent is already running");
            return;
        }

        if let Some(mut agent) = self.agent.take() {
            let _chat_handler = self.chat_handler.clone();
            spawn_local(async move {
                tracing::info!("Starting agent runtime");
                agent.run().await;
            });
            self.running = true;
            tracing::info!("Agent runtime started");
        }
    }

    #[wasm_bindgen]
    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, JsValue> {
        if !self.running {
            return Err(JsValue::from_str(
                "Agent is not running. Call start() first.",
            ));
        }

        // Convert Vec<Message> to Vec<ChatMessage>
        let chat_messages: Vec<ChatMessage> = messages.into_iter().map(|msg| msg.into()).collect();

        let chat = Chat {
            messages: chat_messages,

            // We don't use session_id here
            session_id: 0,
        };

        let mut handler = self.chat_handler.lock().await;
        match handler.chat(chat).await {
            Ok(response) => Ok(response),
            Err(e) => Err(JsValue::from_str(&format!("Chat error: {}", e))),
        }
    }

    #[wasm_bindgen]
    pub fn is_running(&self) -> bool {
        self.running
    }
}

/// Initialize the WASM module.
#[wasm_bindgen(start)]
pub fn start() {
    log::init();
    tracing::info!("WASM module initialized");
}
