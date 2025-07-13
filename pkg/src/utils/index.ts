import { 
  Note, 
  LexicalNode, 
  TextNode, 
  ParagraphNode, 
  Message, 
  ChatAction, 
  BriefNode,
  HeadingNode,
  AIEmbeddingNode
} from '../types';

/**
 * Utility functions for working with Lexical nodes
 */
export const NodeUtils = {
  /**
   * Extract plain text from a lexical node
   */
  extractText(node: LexicalNode): string {
    switch (node.type) {
      case 'text':
        return node.text;
      case 'paragraph':
        return node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'heading':
        return node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'list':
        return node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'listitem':
        return '• ' + node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'quote':
        return node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'code':
        return node.text || (node.children ? node.children.map(child => NodeUtils.extractText(child)).join('') : '');
      case 'link':
        return `${node.children.map(child => NodeUtils.extractText(child)).join('')} (${node.url})`;
      case 'autolink':
        return `${node.children.map(child => NodeUtils.extractText(child)).join('')} (${node.url})`;
      case 'hashtag':
        return node.text;
      case 'table':
        return node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'tablerow':
        return node.children.map(child => NodeUtils.extractText(child)).join(' | ');
      case 'tablecell':
        return node.children.map(child => NodeUtils.extractText(child)).join('');
      case 'page-break':
        return '---';
      case 'ai-embedding':
        return node.content;
      case 'voice-input':
        return node.content;
      case 'chat-message':
        return `[${node.sender}] ${node.content}`;
      case 'chat-session':
        return node.messages.map(msg => `[${msg.sender}] ${msg.content}`).join('\n');
      case 'mention':
        return node.text;
      default:
        return '';
    }
  },

  /**
   * Check if a node is empty (has no meaningful content)
   */
  isEmpty(node: LexicalNode): boolean {
    const text = NodeUtils.extractText(node);
    return text.trim().length === 0;
  },

  /**
   * Get the node type as a human-readable string
   */
  getNodeTypeLabel(node: LexicalNode): string {
    switch (node.type) {
      case 'text':
        return 'Text';
      case 'paragraph':
        return 'Paragraph';
      case 'heading':
        return `Heading ${node.tag.toUpperCase()}`;
      case 'list':
        return node.listType === 'bullet' ? 'Bullet List' : 'Numbered List';
      case 'listitem':
        return 'List Item';
      case 'quote':
        return 'Quote';
      case 'code':
        return 'Code';
      case 'link':
        return 'Link';
      case 'autolink':
        return 'Auto Link';
      case 'hashtag':
        return 'Hashtag';
      case 'table':
        return 'Table';
      case 'tablerow':
        return 'Table Row';
      case 'tablecell':
        return 'Table Cell';
      case 'page-break':
        return 'Page Break';
      case 'ai-embedding':
        return 'AI Embedding';
      case 'voice-input':
        return 'Voice Input';
      case 'chat-message':
        return 'Chat Message';
      case 'chat-session':
        return 'Chat Session';
      case 'mention':
        return 'Mention';
      default:
        return 'Unknown';
    }
  },

  /**
   * Check if a node has children
   */
  hasChildren(node: LexicalNode): boolean {
    return 'children' in node && Array.isArray(node.children) && node.children.length > 0;
  },

  /**
   * Count the number of child nodes (recursively)
   */
  countChildren(node: LexicalNode): number {
    if (!NodeUtils.hasChildren(node)) {
      return 0;
    }
    
    const children = (node as any).children || [];
    return children.reduce((count: number, child: LexicalNode) => {
      return count + 1 + NodeUtils.countChildren(child);
    }, 0);
  },

  /**
   * Find all nodes of a specific type
   */
  findNodesByType<T extends LexicalNode>(node: LexicalNode, type: T['type']): T[] {
    const results: T[] = [];
    
    if (node.type === type) {
      results.push(node as T);
    }
    
    if (NodeUtils.hasChildren(node)) {
      const children = (node as any).children || [];
      children.forEach((child: LexicalNode) => {
        results.push(...NodeUtils.findNodesByType(child, type));
      });
    }
    
    return results;
  },

  /**
   * Create a new text node
   */
  createTextNode(text: string, format: number = 0): TextNode {
    return {
      type: 'text',
      text,
      format,
      detail: 0,
      mode: 'normal',
      style: '',
      version: 1,
    };
  },

  /**
   * Create a new paragraph node
   */
  createParagraphNode(children: LexicalNode[] = []): ParagraphNode {
    return {
      type: 'paragraph',
      children,
      textFormat: 0,
      textStyle: '',
      version: 1,
    };
  },

  /**
   * Create a new heading node
   */
  createHeadingNode(tag: HeadingNode['tag'], children: LexicalNode[] = []): HeadingNode {
    return {
      type: 'heading',
      tag,
      children,
      version: 1,
    };
  },

  /**
   * Create a new AI embedding node
   */
  createAIEmbeddingNode(content: string, isLoading: boolean = false): AIEmbeddingNode {
    return {
      type: 'ai-embedding',
      content,
      isLoading,
      version: 1,
    };
  },
};

/**
 * Utility functions for working with notes
 */
export const NoteUtils = {
  /**
   * Extract all text from a note
   */
  extractText(note: Note): string {
    return note.lexicalState.root.children
      .map(child => NodeUtils.extractText(child))
      .join('\n');
  },

  /**
   * Get brief summary of the note
   */
  getBrief(note: Note): BriefNode[] {
    const briefs: BriefNode[] = [];
    
    note.lexicalState.root.children.forEach((node, index) => {
      const content = NodeUtils.extractText(node);
      if (content.trim().length > 0) {
        briefs.push({
          id: index,
          nodeType: node.type,
          content,
        });
      }
    });
    
    return briefs;
  },

  /**
   * Count words in a note
   */
  countWords(note: Note): number {
    const text = NoteUtils.extractText(note);
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  },

  /**
   * Count characters in a note
   */
  countCharacters(note: Note): number {
    const text = NoteUtils.extractText(note);
    return text.length;
  },

  /**
   * Find all nodes of a specific type in a note
   */
  findNodesByType<T extends LexicalNode>(note: Note, type: T['type']): T[] {
    const results: T[] = [];
    
    note.lexicalState.root.children.forEach(child => {
      results.push(...NodeUtils.findNodesByType(child, type));
    });
    
    return results;
  },

  /**
   * Check if a note is empty
   */
  isEmpty(note: Note): boolean {
    return note.lexicalState.root.children.every(child => NodeUtils.isEmpty(child));
  },

  /**
   * Get node by ID (root child index)
   */
  getNodeById(note: Note, id: number): LexicalNode | null {
    return note.lexicalState.root.children[id] || null;
  },

  /**
   * Create a new empty note
   */
  createEmptyNote(noteId: string): Note {
    return {
      noteId,
      lexicalState: {
        root: {
          type: 'root',
          children: [],
          version: 1,
        },
      },
    };
  },

  /**
   * Add a node to a note
   */
  addNode(note: Note, node: LexicalNode, position?: number): Note {
    const newNote = { ...note };
    const children = [...newNote.lexicalState.root.children];
    
    if (position !== undefined && position >= 0 && position <= children.length) {
      children.splice(position, 0, node);
    } else {
      children.push(node);
    }
    
    newNote.lexicalState.root.children = children;
    return newNote;
  },

  /**
   * Remove a node from a note
   */
  removeNode(note: Note, nodeId: number): Note {
    const newNote = { ...note };
    newNote.lexicalState.root.children = newNote.lexicalState.root.children.filter(
      (_, index) => index !== nodeId
    );
    return newNote;
  },

  /**
   * Update a node in a note
   */
  updateNode(note: Note, nodeId: number, newNode: LexicalNode): Note {
    const newNote = { ...note };
    newNote.lexicalState.root.children = newNote.lexicalState.root.children.map(
      (node, index) => index === nodeId ? newNode : node
    );
    return newNote;
  },
};

/**
 * Utility functions for working with messages
 */
export const MessageUtils = {
  /**
   * Create a new message
   */
  createMessage(content: string, role: string = 'user'): Message {
    return { content, role };
  },

  /**
   * Create a user message
   */
  createUserMessage(content: string): Message {
    return MessageUtils.createMessage(content, 'user');
  },

  /**
   * Create an assistant message
   */
  createAssistantMessage(content: string): Message {
    return MessageUtils.createMessage(content, 'assistant');
  },

  /**
   * Create a system message
   */
  createSystemMessage(content: string): Message {
    return MessageUtils.createMessage(content, 'system');
  },

  /**
   * Filter messages by role
   */
  filterByRole(messages: Message[], role: string): Message[] {
    return messages.filter(msg => msg.role === role);
  },

  /**
   * Get the last message
   */
  getLastMessage(messages: Message[]): Message | null {
    return messages.length > 0 ? (messages[messages.length - 1] || null) : null;
  },

  /**
   * Get the last message by role
   */
  getLastMessageByRole(messages: Message[], role: string): Message | null {
    const filtered = MessageUtils.filterByRole(messages, role);
    return MessageUtils.getLastMessage(filtered);
  },

  /**
   * Count messages by role
   */
  countByRole(messages: Message[], role: string): number {
    return MessageUtils.filterByRole(messages, role).length;
  },

  /**
   * Truncate messages to a maximum count
   */
  truncate(messages: Message[], maxCount: number): Message[] {
    if (messages.length <= maxCount) {
      return messages;
    }
    return messages.slice(-maxCount);
  },

  /**
   * Format message for display
   */
  formatForDisplay(message: Message): string {
    const roleLabel = message.role.charAt(0).toUpperCase() + message.role.slice(1);
    return `${roleLabel}: ${message.content}`;
  },
};

/**
 * Utility functions for working with chat actions
 */
export const ChatActionUtils = {
  /**
   * Check if action is a reply
   */
  isReply(action: ChatAction): action is { action: 'reply'; content: string } {
    return action.action === 'reply';
  },

  /**
   * Check if action is insert node
   */
  isInsertNode(action: ChatAction): action is { action: 'insert_node'; insertAfter: number; nodeType: string; content: string } {
    return action.action === 'insert_node';
  },

  /**
   * Check if action is modify node
   */
  isModifyNode(action: ChatAction): action is { action: 'modify_node'; id: number; nodeType: string; content: string } {
    return action.action === 'modify_node';
  },

  /**
   * Apply a chat action to a note
   */
  applyToNote(note: Note, action: ChatAction): Note {
    if (ChatActionUtils.isInsertNode(action)) {
      // Create a new node based on the action
      let newNode: LexicalNode;
      
      switch (action.nodeType) {
        case 'text':
          newNode = NodeUtils.createTextNode(action.content);
          break;
        case 'paragraph':
          newNode = NodeUtils.createParagraphNode([NodeUtils.createTextNode(action.content)]);
          break;
        case 'heading':
          newNode = NodeUtils.createHeadingNode('h1', [NodeUtils.createTextNode(action.content)]);
          break;
        case 'ai-embedding':
          newNode = NodeUtils.createAIEmbeddingNode(action.content);
          break;
        default:
          newNode = NodeUtils.createParagraphNode([NodeUtils.createTextNode(action.content)]);
      }
      
      return NoteUtils.addNode(note, newNode, action.insertAfter + 1);
    } else if (ChatActionUtils.isModifyNode(action)) {
      // Update existing node
      const existingNode = NoteUtils.getNodeById(note, action.id);
      if (existingNode) {
        let updatedNode: LexicalNode;
        
        switch (action.nodeType) {
          case 'text':
            updatedNode = { ...existingNode, text: action.content } as TextNode;
            break;
          case 'paragraph':
            updatedNode = { 
              ...existingNode, 
              children: [NodeUtils.createTextNode(action.content)] 
            } as ParagraphNode;
            break;
          case 'ai-embedding':
            updatedNode = { 
              ...existingNode, 
              content: action.content 
            } as AIEmbeddingNode;
            break;
          default:
            updatedNode = existingNode;
        }
        
        return NoteUtils.updateNode(note, action.id, updatedNode);
      }
    }
    
    return note;
  },

  /**
   * Get a human-readable description of the action
   */
  getDescription(action: ChatAction): string {
    if (ChatActionUtils.isReply(action)) {
      return 'Agent replied to your message';
    } else if (ChatActionUtils.isInsertNode(action)) {
      return `Agent inserted a new ${action.nodeType} node`;
    } else if (ChatActionUtils.isModifyNode(action)) {
      return `Agent modified ${action.nodeType} node at position ${action.id}`;
    }
    
    return 'Unknown action';
  },
};

/**
 * Utility functions for formatting and validation
 */
export const FormatUtils = {
  /**
   * Format text with basic formatting flags
   */
  formatText(text: string, format: number): string {
    let formatted = text;
    
    if (format & 1) { // Bold
      formatted = `**${formatted}**`;
    }
    if (format & 2) { // Italic
      formatted = `*${formatted}*`;
    }
    if (format & 4) { // Underline
      formatted = `<u>${formatted}</u>`;
    }
    if (format & 8) { // Strikethrough
      formatted = `~~${formatted}~~`;
    }
    
    return formatted;
  },

  /**
   * Parse format flags from text
   */
  parseFormat(text: string): { text: string; format: number } {
    let format = 0;
    let cleanText = text;
    
    // This is a simplified parser - in practice, you'd want more robust parsing
    if (text.includes('**')) {
      format |= 1; // Bold
      cleanText = cleanText.replace(/\*\*/g, '');
    }
    if (text.includes('*')) {
      format |= 2; // Italic
      cleanText = cleanText.replace(/\*/g, '');
    }
    if (text.includes('<u>')) {
      format |= 4; // Underline
      cleanText = cleanText.replace(/<\/?u>/g, '');
    }
    if (text.includes('~~')) {
      format |= 8; // Strikethrough
      cleanText = cleanText.replace(/~~/g, '');
    }
    
    return { text: cleanText, format };
  },

  /**
   * Validate JWT token format
   */
  validateJWT(token: string): boolean {
    // Basic JWT format validation
    const parts = token.split('.');
    return parts.length === 3 && parts.every(part => part.length > 0);
  },

  /**
   * Truncate text to a maximum length
   */
  truncateText(text: string, maxLength: number, suffix: string = '...'): string {
    if (text.length <= maxLength) {
      return text;
    }
    
    return text.substring(0, maxLength - suffix.length) + suffix;
  },

  /**
   * Sanitize HTML content
   */
  sanitizeHtml(html: string): string {
    // Basic HTML sanitization - in practice, use a proper sanitization library
    return html
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#x27;');
  },
};

/**
 * Debug utilities
 */
export const DebugUtils = {
  /**
   * Log note structure
   */
  logNoteStructure(note: Note): void {
    console.log('Note Structure:', {
      id: note.noteId,
      childCount: note.lexicalState.root.children.length,
      children: note.lexicalState.root.children.map((child, index) => ({
        index,
        type: child.type,
        content: NodeUtils.extractText(child).substring(0, 50),
      })),
    });
  },

  /**
   * Log agent status
   */
  logAgentStatus(status: string, error?: any): void {
    console.log(`Agent Status: ${status}`, error ? { error } : {});
  },

  /**
   * Log chat action
   */
  logChatAction(action: ChatAction): void {
    console.log('Chat Action:', {
      type: action.action,
      description: ChatActionUtils.getDescription(action),
      action,
    });
  },
}; 