use std::{future::Future, sync::Arc};

use amico_core::{
    Agent, OnFinish,
    traits::{EventSource, Strategy},
    types::{AgentEvent, Chat, ChatMessage, Interaction},
};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::{
    spawn,
    sync::{Mutex, mpsc},
    task::JoinHandle,
};
use tokio_with_wasm::alias as tokio;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod log;
mod service;

#[derive(Serialize, Deserialize)]
struct EventInner {
    message: String,
    value: i32,
}

#[derive(Debug)]
struct ChatSource {
    chat_rx: Arc<Mutex<mpsc::Receiver<Chat>>>,
    reply_tx: mpsc::Sender<String>,
}

#[derive(Debug)]
struct ChatHandler {
    chat_tx: mpsc::Sender<Chat>,
    reply_rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl ChatHandler {
    pub async fn chat(&mut self, chat: Chat) -> anyhow::Result<String> {
        self.chat_tx.send(chat).await.unwrap_or_else(|err| {
            tracing::error!("Failed to send chat: {}", err);
        });

        let reply = self.reply_rx.lock().await.recv().await.unwrap_or_else(|| {
            tracing::error!("Failed to receive reply: channel closed");
            "Failed to receive reply".to_string()
        });

        Ok(reply)
    }
}

fn create_chat() -> (ChatSource, ChatHandler) {
    let (chat_tx, chat_rx) = mpsc::channel(1);
    let (reply_tx, reply_rx) = mpsc::channel(1);
    (
        ChatSource {
            chat_rx: Arc::new(Mutex::new(chat_rx)),
            reply_tx,
        },
        ChatHandler {
            chat_tx,
            reply_rx: Arc::new(Mutex::new(reply_rx)),
        },
    )
}

impl EventSource for ChatSource {
    fn spawn<F, Fut>(&self, on_event: F) -> JoinHandle<anyhow::Result<()>>
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Option<String>> + Send + 'static,
    {
        let chat_rx = self.chat_rx.clone();
        let reply_tx = self.reply_tx.clone();
        spawn(async move {
            while let Some(chat) = chat_rx.lock().await.recv().await {
                let event =
                    AgentEvent::new("Chat", "ChatSource").interaction(Interaction::Chat(chat));

                // Make the Strategy handle the interaction.
                let reply = on_event(event).await.unwrap_or_else(|| {
                    tracing::warn!("Agent did not reply to interaction");
                    "Agent did not reply to interaction".to_string()
                });

                reply_tx.send(reply).await.unwrap_or_else(|err| {
                    tracing::error!("Failed to send reply: {}", err);
                });
            }

            Ok(())
        })
    }
}

struct AppStrategy;

impl Strategy for AppStrategy {
    async fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        _delegate: amico_core::world::ActionSender<'_>,
    ) -> anyhow::Result<Option<String>> {
        // Extract the interaction from the event.
        // Do not handle non-interaction events now.
        let interaction = agent_event
            .get_interaction()
            .ok_or(anyhow!("Cannot handle non-interaction event"))?;

        match interaction {
            Interaction::Chat(chat) => Ok(Some(format!("Received chat: {:?}", chat))),
            // _ => Ok(None),
        }
    }
}

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
        let (chat_source, chat_handler) = create_chat();
        let mut agent = Agent::new(AppStrategy);
        agent.spawn_event_source(chat_source, OnFinish::Stop);

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

// #[tokio::main(flavor = "current_thread")]
// async fn test_agent() {
//     let mut agent_rt = AgentWasmRuntime::new();
//     agent_rt.start();
//     // Test is now async-friendly but we won't await anything here
// }

#[wasm_bindgen(start)]
pub fn start() {
    log::init();
    // Remove the test_agent call from start() since it's meant for testing
    tracing::info!("WASM module initialized");
}
