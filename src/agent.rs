use std::{future::Future, sync::Arc};

use amico_core::{
    Agent, OnFinish,
    traits::{EventSource, Strategy},
    types::{AgentEvent, Chat, Interaction},
};
use anyhow::anyhow;
use tokio::{
    spawn,
    sync::{Mutex, mpsc},
    task::JoinHandle,
};
use tokio_with_wasm::alias as tokio;

use crate::service::AimoModel;

/// The event source for frontend to send chat to the agent.
#[derive(Debug)]
pub struct ChatSource {
    chat_rx: Arc<Mutex<mpsc::Receiver<Chat>>>,
    reply_tx: mpsc::Sender<String>,
}

/// The handler for communication between frontend and agent.
#[derive(Debug)]
pub struct ChatHandler {
    chat_tx: mpsc::Sender<Chat>,
    reply_rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl ChatHandler {
    /// Send a chat to the agent and wait for the reply.
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

/// Create a chat source and handler.
pub fn create_chat() -> (ChatSource, ChatHandler) {
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

/// The event source for frontend to send chat to the agent.
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

/// The strategy for the agent.
pub struct AppStrategy {
    model: AimoModel,
}

impl AppStrategy {
    pub fn new(jwt: String) -> Self {
        Self {
            model: AimoModel::new(jwt),
        }
    }
}

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
            Interaction::Chat(chat) => Ok(Some(self.model.completion(&chat.messages).await?)),
        }
    }
}

/// Create an agent with a chat source and handler.
pub fn create_agent(jwt: String) -> (Agent<AppStrategy>, ChatHandler) {
    let (chat_source, chat_handler) = create_chat();
    let mut agent = Agent::new(AppStrategy::new(jwt));
    agent.spawn_event_source(chat_source, OnFinish::Stop);
    (agent, chat_handler)
}
