use std::{future::Future, sync::Arc};

use amico_core::{
    traits::{EventSource, Strategy}, types::{AgentEvent, Chat, ChatMessage, Interaction}, Agent, OnFinish
};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::{
    spawn,
    sync::{Mutex, mpsc},
    task::JoinHandle,
};
use tokio_with_wasm::alias as tokio;

use crate::{note::{BriefNode, Note}, service::AimoModel};

pub fn get_system_prompt(brief_note: Vec<BriefNode>) -> anyhow::Result<String> {
    let brief_note_str = serde_json::to_string(&brief_note)?;
    let prompt = format!(
        "You are a helpful assistant, AiMo, that can help with note-taking.

        ## Environment Inspection
        
        Here's the structured note the user is working on: {brief_note_str}

        ## Your Task

        You are given the content of the note that the user is working on, and the messages you have had with the user.
        You need to chat with the user to determine what they want to do with the note.
        When you have determined what the user wants to do, you need to take actions to help the user.

        ## Rules

        - You must always reply to the user in the same language as the user's messages.
        - For the `insert_node` and `modify_node` actions, you must always reply with a JSON string, and **DO NOT** include any other text or the code frame.
        - If you find you have already take an action in the messages but the user wants you to modify your action, just re-generate the action based on the original note content.
        
        ## Available Actions

        ### Reply to the user

        If you can't determine what the user wants to do, you can reply to the user with a message to request more information.

        For example:

        ```
        Hello, I'm AiMo, your note-taking assistant. What would you like to do with the note?
        ```

        Or:

        ```
        {{
            \"action\": \"reply\",
            \"content\": \"Hello, I'm AiMo, your note-taking assistant. What would you like to do with the note?\"
        }}
        ```

        ### Insert a new node

        You can insert a new node after a specific node.

        Reply to the user with the following JSON format, but remember: Just reply
        with a raw JSON string, do not include any other text or the code frame.
        
        For example:

        ```
        {{
            \"action\": \"insert_node\",
            \"insert_after\": 0,
            \"node_type\": \"text\",
            \"content\": \"Hello, world!\"
        }}
        ```

        ### Modify a node

        You can modify a specific node.

        Reply to the user with the following JSON format, but remember: Just reply
        with a raw JSON string, do not include any other text or the code frame.
        
        For example:
        
        ```
        {{
            \"action\": \"modify_node\",
            \"id\": 0,
            \"node_type\": \"text\",
            \"content\": \"Hello, world!\"
        }}
        ```
        ",
    );
    Ok(prompt)
}

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
    pub async fn chat(&mut self, chat: Chat, ctx: &ChatContext) -> anyhow::Result<ChatAction> {
        // Add the system prompt to the chat.
        let mut messages = Vec::new();
        messages.push(ChatMessage {
            content: get_system_prompt(ctx.note.get_brief())?,
            role: "system".to_string(),
        });
        messages.extend(chat.messages);

        // Create a new chat with the system prompt.
        let chat = Chat {
            messages,
            session_id: chat.session_id,
        };

        // Send the chat to the agent.
        self.chat_tx.send(chat).await.unwrap_or_else(|err| {
            tracing::error!("Failed to send chat: {}", err);
        });

        // Receive the reply from the agent.
        let reply = self.reply_rx.lock().await.recv().await.unwrap_or_else(|| {
            tracing::error!("Failed to receive reply: channel closed");
            "Failed to receive reply".to_string()
        });

        // Parse the reply to a chat action.
        Ok(ChatAction::try_from_reply(reply)?)
    }
}

/// The context for the agent.
///
/// It contains the note that the agent is working on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContext {
    pub note: Note,
    pub cursor_position: usize,
}

/// The action for the agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatAction {
    /// The action to reply to the chat.
    Reply(Reply),
    /// The action to insert a new node.
    InsertNode(InsertNode),
    /// The action to modify a node.
    ModifyNode(ModifyNode),
}

impl ChatAction {
    /// Parse the reply to a chat action.
    pub fn try_from_reply(reply: String) -> anyhow::Result<Self> {
        // If the reply starts with '```', the agent mistakenly replied with code frame.
        // So we need to remove the code frame.
        let reply = reply.trim_start_matches("```").trim_end_matches("```");

        // If the reply starts with `{`, it's a JSON string. Try to parse it.
        if reply.starts_with("{") {
            let action: serde_json::Value = serde_json::from_str(reply)?;

            // Try to parse the action.
            return action.get("action").map(|action| {
                match action.as_str() {
                    Some("insert_node") => Ok(Self::InsertNode(serde_json::from_value::<InsertNode>(action.clone())?)),
                    Some("modify_node") => Ok(Self::ModifyNode(serde_json::from_value::<ModifyNode>(action.clone())?)),

                    // If the agent choose to reply in an action, we can also handle it.
                    Some("reply") => Ok(Self::Reply(serde_json::from_value::<Reply>(action.clone())?)),

                    // The action type is not supported. Do not treat this as a reply.
                    // Report the error to the agent.
                    _ => Err(anyhow!("Invalid action: {}", action)),
                }
            })
            // Default to reply if the `action` field is not found.
            .unwrap_or(Ok(Self::Reply(reply.into())));
        }

        // If the reply does not start with `{`, it's a normal text reply.
        // So we can just return it as a reply.
        Ok(Self::Reply(reply.into()))
    }
}

/// The action to reply to the chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub action: String,
    pub content: String,
}

impl From<String> for Reply {
    fn from(content: String) -> Self {
        Self {
            action: "reply".to_string(),
            content,
        }
    }
}

impl From<&str> for Reply {
    fn from(content: &str) -> Self {
        Self {
            action: "reply".to_string(),
            content: content.to_string(),
        }
    }  
}

/// The action to insert a new node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertNode {
    pub action: String,
    pub insert_after: usize,
    pub node_type: String,
    pub content: String,
}

/// The action to modify a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyNode {
    pub action: String,
    pub id: usize,
    pub node_type: String,
    pub content: String,
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
