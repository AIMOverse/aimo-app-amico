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

use agent::{AppStrategy, ChatHandler, create_agent};

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
    pub fn new() -> AgentWasmRuntime {
        let (agent, chat_handler) = create_agent();

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
    pub async fn send_chat(&self, message: String) -> Result<String, JsValue> {
        if !self.running {
            return Err(JsValue::from_str(
                "Agent is not running. Call start() first.",
            ));
        }

        let chat_message = ChatMessage {
            content: message,
            ..Default::default()
        };
        let chat = Chat {
            messages: vec![chat_message],
            ..Default::default()
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
