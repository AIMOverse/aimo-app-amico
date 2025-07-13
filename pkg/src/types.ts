// TypeScript type definitions for the WASM agent

export type TextDirection = 'ltr' | 'rtl';

export interface BaseNodeProperties {
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface TextNode {
  type: 'text';
  text: string;
  format: number; // Binary flags: 1=bold, 2=italic, 4=underline, 8=strikethrough
  detail: number;
  mode: string;
  style: string;
  version: number;
  direction?: TextDirection;
  indent?: number;
}

export interface ParagraphNode {
  type: 'paragraph';
  children: LexicalNode[];
  textFormat: number;
  textStyle: string;
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface HeadingNode {
  type: 'heading';
  tag: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6';
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface ListNode {
  type: 'list';
  listType: 'bullet' | 'number';
  start?: number;
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface ListItemNode {
  type: 'listitem';
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface QuoteNode {
  type: 'quote';
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface CodeNode {
  type: 'code';
  text?: string;
  language?: string;
  children?: LexicalNode[];
  format: number;
  version: number;
  direction?: TextDirection;
  indent?: number;
}

export interface LinkNode {
  type: 'link';
  url: string;
  rel?: string;
  target?: string;
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface AutoLinkNode {
  type: 'autolink';
  url: string;
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface HashtagNode {
  type: 'hashtag';
  text: string;
  format: number;
  version: number;
  direction?: TextDirection;
  indent?: number;
}

export interface TableNode {
  type: 'table';
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface TableRowNode {
  type: 'tablerow';
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface TableCellNode {
  type: 'tablecell';
  children: LexicalNode[];
  headerState: number;
  colSpan: number;
  rowSpan: number;
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface PageBreakNode {
  type: 'page-break';
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface AIEmbeddingNode {
  type: 'ai-embedding';
  content: string;
  isLoading: boolean;
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface VoiceInputNode {
  type: 'voice-input';
  content: string;
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface ChatMessageNode {
  type: 'chat-message';
  sender: 'user' | 'agent' | 'system';
  content: string;
  timestamp: string;
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface ChatSessionNode {
  type: 'chat-session';
  sessionId: string;
  isActive: boolean;
  messages: ChatSessionMessage[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface ChatSessionMessage {
  id: number;
  sender: 'user' | 'agent' | 'system';
  content: string;
  timestamp: string;
}

export interface MentionNode {
  type: 'mention';
  mentionName: string;
  text: string;
  format: number;
  version: number;
  direction?: TextDirection;
  indent?: number;
}

export type LexicalNode = 
  | TextNode
  | ParagraphNode
  | HeadingNode
  | ListNode
  | ListItemNode
  | QuoteNode
  | CodeNode
  | LinkNode
  | AutoLinkNode
  | HashtagNode
  | TableNode
  | TableRowNode
  | TableCellNode
  | PageBreakNode
  | AIEmbeddingNode
  | VoiceInputNode
  | ChatMessageNode
  | ChatSessionNode
  | MentionNode;

export interface RootNode {
  type: 'root';
  children: LexicalNode[];
  version: number;
  direction?: TextDirection;
  format?: string;
  indent?: number;
}

export interface LexicalState {
  root: RootNode;
}

export interface Note {
  noteId: string;
  lexicalState: LexicalState;
}

export interface BriefNode {
  id: number;
  nodeType: string;
  content: string;
}

export interface Message {
  content: string;
  role: string;
}

export interface ChatContext {
  note: Note;
  cursorPosition: number;
}

export interface Reply {
  action: 'reply';
  content: string;
}

export interface InsertNode {
  action: 'insert_node';
  insertAfter: number;
  nodeType: string;
  content: string;
}

export interface ModifyNode {
  action: 'modify_node';
  id: number;
  nodeType: string;
  content: string;
}

export type ChatAction = Reply | InsertNode | ModifyNode;

// WASM Runtime types
export interface AgentWasmRuntime {
  new(jwt: string): AgentWasmRuntime;
  start(): void;
  chat(messages: Message[], cursorPosition: number, note: Note): Promise<ChatAction>;
  isRunning(): boolean;
}

// Agent status
export type AgentStatus = 'idle' | 'starting' | 'running' | 'error';

// Error types
export interface AgentError {
  message: string;
  code?: string;
}

// Hook configuration
export interface AgentConfig {
  jwt: string;
  autoStart?: boolean;
}

// Chat configuration
export interface ChatConfig {
  maxMessages?: number;
  timeout?: number;
} 