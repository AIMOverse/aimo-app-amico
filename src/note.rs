use serde::{Deserialize, Serialize};

/// Main Note structure representing a complete note with metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: Option<i32>,
    pub note_id: String,
    pub lexical_state: LexicalState,
}

/// The top-level Lexical state containing the root node
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LexicalState {
    pub root: RootNode,
}

/// Base properties shared by all nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseNodeProperties {
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TextDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<u32>,
}

/// Text direction enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextDirection {
    #[serde(rename = "ltr")]
    LeftToRight,
    #[serde(rename = "rtl")]
    RightToLeft,
}

/// Root node - the top-level container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootNode {
    #[serde(rename = "type")]
    pub node_type: String, // Always "root"
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Main node enumeration covering all possible node types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LexicalNode {
    #[serde(rename = "text")]
    Text(TextNode),
    #[serde(rename = "paragraph")]
    Paragraph(ParagraphNode),
    #[serde(rename = "heading")]
    Heading(HeadingNode),
    #[serde(rename = "list")]
    List(ListNode),
    #[serde(rename = "listitem")]
    ListItem(ListItemNode),
    #[serde(rename = "quote")]
    Quote(QuoteNode),
    #[serde(rename = "code")]
    Code(CodeNode),
    #[serde(rename = "link")]
    Link(LinkNode),
    #[serde(rename = "autolink")]
    AutoLink(AutoLinkNode),
    #[serde(rename = "hashtag")]
    Hashtag(HashtagNode),
    #[serde(rename = "table")]
    Table(TableNode),
    #[serde(rename = "tablerow")]
    TableRow(TableRowNode),
    #[serde(rename = "tablecell")]
    TableCell(TableCellNode),
    #[serde(rename = "page-break")]
    PageBreak(PageBreakNode),
    // Custom node types
    #[serde(rename = "ai-embedding")]
    AIEmbedding(AIEmbeddingNode),
    #[serde(rename = "voice-input")]
    VoiceInput(VoiceInputNode),
    #[serde(rename = "chat-message")]
    ChatMessage(ChatMessageNode),
    #[serde(rename = "chat-session")]
    ChatSession(ChatSessionNode),
    #[serde(rename = "mention")]
    Mention(MentionNode),
}

/// Text node - basic text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub text: String,
    pub format: u32, // Binary flags: 1=bold, 2=italic, 4=underline, 8=strikethrough
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Paragraph node - container for text and inline elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphNode {
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Heading node - document headings (h1-h6)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingNode {
    pub tag: HeadingTag,
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Heading tag enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HeadingTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// List node - ordered or unordered lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNode {
    #[serde(rename = "listType")]
    pub list_type: ListType,
    pub start: Option<u32>,
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// List type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    Bullet,
    Number,
}

/// List item node - individual items within lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemNode {
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Quote node - blockquotes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteNode {
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Code node - inline code or code blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>, // For inline code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>, // For code blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<LexicalNode>>, // For code blocks
    pub format: u32,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Link node - hyperlinks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkNode {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Auto link node - automatically detected links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoLinkNode {
    pub url: String,
    pub children: Vec<LexicalNode>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Hashtag node - hashtags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashtagNode {
    pub text: String,
    pub format: u32,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Table node - tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNode {
    pub children: Vec<LexicalNode>, // TableRow nodes
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Table row node - table rows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRowNode {
    pub children: Vec<LexicalNode>, // TableCell nodes
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Table cell node - table cells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCellNode {
    pub children: Vec<LexicalNode>,
    #[serde(rename = "headerState")]
    pub header_state: u32,
    #[serde(rename = "colSpan")]
    pub col_span: u32,
    #[serde(rename = "rowSpan")]
    pub row_span: u32,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Page break node - page breaks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBreakNode {
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// AI Embedding node - AI-generated content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEmbeddingNode {
    pub content: String,
    #[serde(rename = "isLoading")]
    pub is_loading: bool,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Voice Input node - voice-to-text transcriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInputNode {
    pub content: String,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Chat Message node - individual chat messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageNode {
    pub sender: MessageSender,
    pub content: String,
    pub timestamp: String, // ISO timestamp
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Message sender enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageSender {
    User,
    Agent,
    System,
}

/// Chat Session node - chat sessions containing multiple messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSessionNode {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub messages: Vec<ChatSessionMessage>,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Message within a chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSessionMessage {
    pub id: u32,
    pub sender: MessageSender,
    pub content: String,
    pub timestamp: String, // ISO timestamp
}

/// Mention node - @mentions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionNode {
    #[serde(rename = "mentionName")]
    pub mention_name: String,
    pub text: String,
    pub format: u32,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}
