use serde::{Deserialize, Serialize};

/// Main Note structure representing a complete note with metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // Serde ignores unknown fields by default
pub struct Note {
    pub note_id: Option<String>,
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
    // Additional fields found in the example JSON
    #[serde(default)]
    pub detail: u32,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub style: String,
    #[serde(flatten)]
    pub base: BaseNodeProperties,
}

/// Paragraph node - container for text and inline elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphNode {
    pub children: Vec<LexicalNode>,
    // Additional fields found in the example JSON
    #[serde(default, rename = "textFormat")]
    pub text_format: u32,
    #[serde(default, rename = "textStyle")]
    pub text_style: String,
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
    // This field is unnecessary and not used in the code.
    // #[serde(rename = "isActive")]
    // pub is_active: bool,
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

/// Brief node - For the agent to work on.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BriefNode {
    pub id: usize,
    pub node_type: String,
    pub content: String,
}

impl Note {
    /// Get the briefs for the note.
    pub fn get_brief(&self) -> Vec<BriefNode> {
        let mut briefs = Vec::new();
        
        // Process each root node with its index
        for (index, node) in self.lexical_state.root.children.iter().enumerate() {
            self.collect_brief_from_node(node, &mut briefs, index);
        }
        
        briefs
    }
    
    /// Collect brief from a single node using its root index
    fn collect_brief_from_node(&self, node: &LexicalNode, briefs: &mut Vec<BriefNode>, root_index: usize) {
        let (node_type, content) = match node {
            LexicalNode::Text(text_node) => {
                ("text", text_node.text.clone())
            }
            LexicalNode::Paragraph(para) => {
                let content = self.extract_text_from_nodes(&para.children);
                ("paragraph", content)
            }
            LexicalNode::Heading(heading) => {
                let content = self.extract_text_from_nodes(&heading.children);
                ("heading", content)
            }
            LexicalNode::List(list) => {
                let content = self.extract_text_from_nodes(&list.children);
                ("list", content)
            }
            LexicalNode::ListItem(item) => {
                let content = self.extract_text_from_nodes(&item.children);
                ("listitem", content)
            }
            LexicalNode::Quote(quote) => {
                let content = self.extract_text_from_nodes(&quote.children);
                ("quote", content)
            }
            LexicalNode::Code(code) => {
                let content = if let Some(text) = &code.text {
                    text.clone()
                } else if let Some(children) = &code.children {
                    self.extract_text_from_nodes(children)
                } else {
                    String::new()
                };
                ("code", content)
            }
            LexicalNode::Link(link) => {
                let content = format!("{} ({})", self.extract_text_from_nodes(&link.children), link.url);
                ("link", content)
            }
            LexicalNode::AutoLink(auto_link) => {
                let content = format!("{} ({})", self.extract_text_from_nodes(&auto_link.children), auto_link.url);
                ("autolink", content)
            }
            LexicalNode::Hashtag(hashtag) => {
                ("hashtag", hashtag.text.clone())
            }
            LexicalNode::Table(table) => {
                let content = self.extract_text_from_nodes(&table.children);
                ("table", content)
            }
            LexicalNode::TableRow(row) => {
                let content = self.extract_text_from_nodes(&row.children);
                ("tablerow", content)
            }
            LexicalNode::TableCell(cell) => {
                let content = self.extract_text_from_nodes(&cell.children);
                ("tablecell", content)
            }
            LexicalNode::PageBreak(_) => {
                ("page-break", "---".to_string())
            }
            LexicalNode::AIEmbedding(ai) => {
                ("ai-embedding", ai.content.clone())
            }
            LexicalNode::VoiceInput(voice) => {
                ("voice-input", voice.content.clone())
            }
            LexicalNode::ChatMessage(msg) => {
                let content = format!("[{}] {}", msg.sender.to_string(), msg.content);
                ("chat-message", content)
            }
            LexicalNode::ChatSession(session) => {
                let content = session.messages.iter()
                    .map(|msg| format!("[{}] {}", msg.sender.to_string(), msg.content))
                    .collect::<Vec<_>>()
                    .join("\n");
                ("chat-session", content)
            }
            LexicalNode::Mention(mention) => {
                ("mention", mention.text.clone())
            }
        };
        
        // Only add non-empty content to briefs
        if !content.trim().is_empty() {
            briefs.push(BriefNode {
                id: root_index,
                node_type: node_type.to_string(),
                content,
            });
        }
    }
    
    /// Helper method to recursively extract text from nodes
    fn extract_text_from_nodes(&self, nodes: &[LexicalNode]) -> String {
        let mut text = String::new();
        
        for node in nodes {
            match node {
                LexicalNode::Text(text_node) => {
                    text.push_str(&text_node.text);
                }
                LexicalNode::Paragraph(para) => {
                    text.push_str(&self.extract_text_from_nodes(&para.children));
                }
                LexicalNode::Heading(heading) => {
                    text.push_str(&self.extract_text_from_nodes(&heading.children));
                }
                LexicalNode::List(list) => {
                    text.push_str(&self.extract_text_from_nodes(&list.children));
                }
                LexicalNode::ListItem(item) => {
                    text.push_str("• ");
                    text.push_str(&self.extract_text_from_nodes(&item.children));
                }
                LexicalNode::Quote(quote) => {
                    text.push_str(&self.extract_text_from_nodes(&quote.children));
                }
                LexicalNode::Code(code) => {
                    if let Some(code_text) = &code.text {
                        text.push_str(code_text);
                    }
                    if let Some(children) = &code.children {
                        text.push_str(&self.extract_text_from_nodes(children));
                    }
                }
                LexicalNode::Link(link) => {
                    text.push_str(&self.extract_text_from_nodes(&link.children));
                }
                LexicalNode::AutoLink(auto_link) => {
                    text.push_str(&self.extract_text_from_nodes(&auto_link.children));
                }
                LexicalNode::Hashtag(hashtag) => {
                    text.push_str(&hashtag.text);
                }
                LexicalNode::Table(table) => {
                    text.push_str(&self.extract_text_from_nodes(&table.children));
                }
                LexicalNode::TableRow(row) => {
                    text.push_str(&self.extract_text_from_nodes(&row.children));
                }
                LexicalNode::TableCell(cell) => {
                    text.push_str(&self.extract_text_from_nodes(&cell.children));
                }
                LexicalNode::AIEmbedding(ai) => {
                    text.push_str(&ai.content);
                }
                LexicalNode::VoiceInput(voice) => {
                    text.push_str(&voice.content);
                }
                LexicalNode::ChatMessage(msg) => {
                    text.push_str(&msg.content);
                }
                LexicalNode::ChatSession(session) => {
                    for msg in &session.messages {
                        text.push_str(&msg.content);
                    }
                }
                LexicalNode::Mention(mention) => {
                    text.push_str(&mention.text);
                }
                LexicalNode::PageBreak(_) => {
                    // Page breaks don't contribute to text content
                }
            }
        }
        
        text
    }
}

// Helper implementation for MessageSender to string conversion
impl std::fmt::Display for MessageSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageSender::User => write!(f, "user"),
            MessageSender::Agent => write!(f, "agent"),
            MessageSender::System => write!(f, "system"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::collections::HashSet;

    #[test]
    fn test_parse_example_note() {
        // Read the example JSON file
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        // Parse the JSON into our Note struct
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        // Test basic structure
        assert_eq!(note.note_id, Some("ef454d5a-2474-4ac1-907a-61cb62de2f9b".to_string()));
        assert_eq!(note.lexical_state.root.node_type, "root");
        assert_eq!(note.lexical_state.root.base.version, 1);
        
        // Test that we have the expected number of children
        let children_count = note.lexical_state.root.children.len();
        assert!(children_count > 0, "Root should have children");
        
        // Test specific node types and content
        let mut _text_nodes = 0;
        let mut paragraph_nodes = 0;
        let mut ai_embedding_nodes = 0;
        
        for child in &note.lexical_state.root.children {
            match child {
                LexicalNode::Text(_) => _text_nodes += 1,
                LexicalNode::Paragraph(para) => {
                    paragraph_nodes += 1;
                    // Test that paragraph has expected structure
                    assert_eq!(para.base.version, 1);
                    if !para.children.is_empty() {
                        // Check first text node in first paragraph
                        if let LexicalNode::Text(text_node) = &para.children[0] {
                            assert!(!text_node.text.is_empty());
                            assert_eq!(text_node.base.version, 1);
                        }
                    }
                }
                LexicalNode::AIEmbedding(ai) => {
                    ai_embedding_nodes += 1;
                    // Test AI embedding structure
                    assert_eq!(ai.base.version, 1);
                    assert!(!ai.content.is_empty() || ai.is_loading);
                }
                _ => {}
            }
        }
        
        // Verify we found the expected node types
        assert!(paragraph_nodes > 0, "Should have found paragraph nodes");
        assert!(ai_embedding_nodes > 0, "Should have found AI embedding nodes");
        
        // Test specific content from the example
        let mut found_hiii = false;
        let mut found_test = false;
        
        for child in &note.lexical_state.root.children {
            if let LexicalNode::Paragraph(para) = child {
                for text_child in &para.children {
                    if let LexicalNode::Text(text_node) = text_child {
                        if text_node.text == "HIII" {
                            found_hiii = true;
                        }
                        if text_node.text == "test" {
                            found_test = true;
                        }
                    }
                }
            }
        }
        
        assert!(found_hiii, "Should find 'HIII' text in the note");
        assert!(found_test, "Should find 'test' text in the note");
        
        println!("✓ Successfully parsed example note with {} children", children_count);
        println!("✓ Found {} paragraph nodes", paragraph_nodes);
        println!("✓ Found {} AI embedding nodes", ai_embedding_nodes);
    }
    
    #[test]
    fn test_parse_specific_ai_embedding() {
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        // Find a specific AI embedding node
        let mut found_loading_ai = false;
        let mut found_content_ai = false;
        
        for child in &note.lexical_state.root.children {
            if let LexicalNode::AIEmbedding(ai) = child {
                if ai.is_loading && ai.content.is_empty() {
                    found_loading_ai = true;
                }
                if !ai.is_loading && ai.content.contains("HIII") {
                    found_content_ai = true;
                }
            }
        }
        
        assert!(found_loading_ai, "Should find a loading AI embedding");
        assert!(found_content_ai, "Should find AI embedding with HIII content");
    }
    
    #[test]
    fn test_parse_text_node_extra_fields() {
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        // Find a text node and verify extra fields are handled
        let mut found_text_with_extra_fields = false;
        
        for child in &note.lexical_state.root.children {
            if let LexicalNode::Paragraph(para) = child {
                for text_child in &para.children {
                    if let LexicalNode::Text(text_node) = text_child {
                        // Check that extra fields are present and parsed correctly
                        assert_eq!(text_node.detail, 0);
                        assert_eq!(text_node.mode, "normal");
                        assert_eq!(text_node.style, "");
                        found_text_with_extra_fields = true;
                        break;
                    }
                }
                if found_text_with_extra_fields {
                    break;
                }
            }
        }
        
        assert!(found_text_with_extra_fields, "Should find text node with extra fields");
    }
    
    #[test]
    fn test_parse_paragraph_extra_fields() {
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        // Find a paragraph node and verify extra fields are handled
        let mut found_paragraph_with_extra_fields = false;
        
        for child in &note.lexical_state.root.children {
            if let LexicalNode::Paragraph(para) = child {
                // Check that extra fields are present and parsed correctly
                assert_eq!(para.text_format, 0);
                assert_eq!(para.text_style, "");
                found_paragraph_with_extra_fields = true;
                break;
            }
        }
        
        assert!(found_paragraph_with_extra_fields, "Should find paragraph node with extra fields");
    }
    
    #[test]
    fn test_roundtrip_serialization() {
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        // Serialize back to JSON
        let serialized = serde_json::to_string(&note)
            .expect("Should be able to serialize note");
        
        // Parse it again
        let reparsed: Note = serde_json::from_str(&serialized)
            .expect("Should be able to parse serialized note");
        
        // Verify key fields are preserved
        assert_eq!(note.note_id, reparsed.note_id);
        assert_eq!(note.lexical_state.root.node_type, reparsed.lexical_state.root.node_type);
        assert_eq!(note.lexical_state.root.children.len(), reparsed.lexical_state.root.children.len());
        
        println!("✓ Successfully completed roundtrip serialization test");
    }
    
    #[test]
    fn test_empty_root_nodes() {
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        println!("Total root children: {}", note.lexical_state.root.children.len());
        
        // Examine each root node to see which ones are "empty"
        for (index, child) in note.lexical_state.root.children.iter().enumerate() {
            let (node_type, content) = match child {
                LexicalNode::Paragraph(para) => {
                    let content = note.extract_text_from_nodes(&para.children);
                    ("paragraph", content)
                }
                LexicalNode::AIEmbedding(ai) => {
                    ("ai-embedding", ai.content.clone())
                }
                LexicalNode::Text(text) => {
                    ("text", text.text.clone())
                }
                _ => ("other", "N/A".to_string())
            };
            
            let is_empty = content.trim().is_empty();
            println!("Root[{}]: {} - empty: {} - content: '{}'", 
                    index, node_type, is_empty, 
                    content.chars().take(30).collect::<String>());
        }
        
        // Now get briefs and show which indices are missing
        let briefs = note.get_brief();
        let brief_ids: HashSet<usize> = briefs.iter().map(|b| b.id).collect();
        
        println!("\nBrief node IDs: {:?}", brief_ids);
        println!("Missing indices (empty nodes):");
        for i in 0..note.lexical_state.root.children.len() {
            if !brief_ids.contains(&i) {
                println!("  Index {} is missing (was empty)", i);
            }
        }
    }

    #[test]
    fn test_get_brief() {
        let json_content = fs::read_to_string("assets/example_note.json")
            .expect("Should be able to read assets/example_note.json");
        
        let note: Note = serde_json::from_str(&json_content)
            .expect("Should be able to parse example note JSON");
        
        // Get brief nodes
        let briefs = note.get_brief();
        
        // Verify we have brief nodes
        assert!(!briefs.is_empty(), "Should have brief nodes");
        
        // Test brief node structure
        for brief in &briefs {
            assert!(!brief.node_type.is_empty(), "Brief node type should not be empty");
            assert!(!brief.content.trim().is_empty(), "Brief content should not be empty");
        }
        
        // Test that we have different node types
        let node_types: HashSet<&String> = briefs.iter().map(|b| &b.node_type).collect();
        assert!(node_types.len() > 1, "Should have multiple node types");
        
        // Test that we have both paragraph and AI embedding nodes
        let has_paragraph = briefs.iter().any(|b| b.node_type == "paragraph");
        let has_ai_embedding = briefs.iter().any(|b| b.node_type == "ai-embedding");
        
        assert!(has_paragraph, "Should have paragraph brief nodes");
        assert!(has_ai_embedding, "Should have AI embedding brief nodes");
        
        // Test specific content
        let has_hiii = briefs.iter().any(|b| b.content.contains("HIII"));
        let has_test = briefs.iter().any(|b| b.content.contains("test"));
        
        assert!(has_hiii, "Should find 'HIII' in brief content");
        assert!(has_test, "Should find 'test' in brief content");
        
        // Test that IDs correspond to root node indices
        let root_children_count = note.lexical_state.root.children.len();
        for brief in &briefs {
            assert!(brief.id < root_children_count, "Brief ID {} should be less than root children count {}", brief.id, root_children_count);
        }
        
        // Test that IDs are unique (no duplicates for different root nodes)
        let mut found_ids = HashSet::new();
        for brief in &briefs {
            assert!(found_ids.insert(brief.id), "Brief IDs should be unique");
        }
        
        println!("✓ Successfully created {} brief nodes", briefs.len());
        println!("✓ Found {} different node types", node_types.len());
        println!("✓ Root has {} children", root_children_count);
        
        // Print first few briefs for verification
        for (i, brief) in briefs.iter().take(5).enumerate() {
            println!("Brief {}: id={}, type={}, content={}", i, brief.id, brief.node_type, brief.content.chars().take(50).collect::<String>());
        }
    }
}
