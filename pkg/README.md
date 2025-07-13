# @aimoverse/note-agent-react

A React TypeScript SDK for interacting with the AIMO note agent WASM module. This library provides hooks and components for chat functionality, note manipulation, and real-time agent interactions.

## Features

- 🚀 **React Hooks**: Easy-to-use hooks for agent interaction
- 📝 **Note Management**: Full support for Lexical note structure
- 💬 **Chat Interface**: Real-time chat with the AI agent
- 🔄 **State Management**: Global agent state with React Context
- 📦 **TypeScript**: Full type safety with comprehensive type definitions
- 🎯 **Tree Shaking**: Optimized bundle size with ES modules
- 🛠️ **Utilities**: Helper functions for note and message manipulation

## Installation

```bash
npm install @aimoverse/note-agent-react
```

## Quick Start

### 1. Wrap your app with the AgentProvider

```tsx
import React from 'react';
import { AgentProvider } from '@aimoverse/note-agent-react';

function App() {
  return (
    <AgentProvider 
      config={{ 
        jwt: 'your-jwt-token',
        autoStart: true 
      }}
    >
      <YourAppContent />
    </AgentProvider>
  );
}
```

### 2. Use the agent in your components

```tsx
import React from 'react';
import { useAgent, useChat, NoteUtils } from '@aimoverse/note-agent-react';

function ChatComponent() {
  const { status, isReady } = useAgent();
  const { sendMessage, messages, isLoading } = useChat();

  const handleSendMessage = async () => {
    const note = NoteUtils.createEmptyNote('example-note');
    const action = await sendMessage(
      'Hello, agent!',
      0, // cursor position
      note
    );
    
    if (action) {
      console.log('Agent action:', action);
    }
  };

  return (
    <div>
      <div>Status: {status}</div>
      <button 
        onClick={handleSendMessage} 
        disabled={!isReady || isLoading}
      >
        Send Message
      </button>
      <div>
        {messages.map((msg, index) => (
          <div key={index}>
            <strong>{msg.role}:</strong> {msg.content}
          </div>
        ))}
      </div>
    </div>
  );
}
```

## API Reference

### Components

#### AgentProvider

The main provider component that manages the global agent state.

```tsx
interface AgentProviderProps {
  children: React.ReactNode;
  config?: AgentConfig;
  onError?: (error: AgentError) => void;
  onStatusChange?: (status: AgentStatus) => void;
}
```

### Hooks

#### useAgent(config?: AgentConfig)

Main hook for interacting with the agent.

```tsx
const {
  agent,           // AgentWasmRuntime instance
  status,          // 'idle' | 'starting' | 'running' | 'error'
  error,           // AgentError | null
  isReady,         // boolean
  isInitialized,   // boolean
  initialize,      // (config: AgentConfig) => Promise<void>
  start,           // () => Promise<void>
  stop,            // () => void
  clearError,      // () => void
  chat,            // (messages, cursorPosition, note) => Promise<ChatAction>
} = useAgent();
```

#### useChat(config?: ChatConfig)

Hook for managing chat conversations.

```tsx
const {
  messages,        // Message[]
  isLoading,       // boolean
  error,           // AgentError | null
  lastMessage,     // Message | null
  sendMessage,     // (content, cursorPosition, note, role?) => Promise<ChatAction>
  addMessage,      // (message: Message) => void
  clearMessages,   // () => void
  clearError,      // () => void
} = useChat();
```

#### useAgentStatus()

Hook for monitoring agent status.

```tsx
const {
  status,          // AgentStatus
  error,           // AgentError | null
  isReady,         // boolean
  statusHistory,   // AgentStatus[]
  isIdle,          // boolean
  isStarting,      // boolean
  isRunning,       // boolean
  hasError,        // boolean
} = useAgentStatus();
```

#### useMessages(initialMessages?: Message[])

Hook for managing message collections.

```tsx
const {
  messages,        // Message[]
  addMessage,      // (message: Message) => void
  removeMessage,   // (index: number) => void
  updateMessage,   // (index: number, message: Message) => void
  clearMessages,   // () => void
  getMessagesByRole, // (role: string) => Message[]
  userMessages,    // Message[]
  assistantMessages, // Message[]
  systemMessages,  // Message[]
  isEmpty,         // boolean
  count,           // number
} = useMessages();
```

#### useConversation(config?: ChatConfig & { persistKey?: string })

Advanced hook for managing conversation history with persistence.

```tsx
const {
  ...chatHookValues,
  conversationHistory,    // Message[][]
  startNewConversation,   // () => void
  loadConversation,       // (index: number) => void
  deleteConversation,     // (index: number) => void
  clearAllConversations,  // () => void
  hasHistory,             // boolean
  conversationCount,      // number
} = useConversation();
```

### Utilities

#### NodeUtils

Utilities for working with Lexical nodes.

```tsx
NodeUtils.extractText(node: LexicalNode): string
NodeUtils.isEmpty(node: LexicalNode): boolean
NodeUtils.getNodeTypeLabel(node: LexicalNode): string
NodeUtils.hasChildren(node: LexicalNode): boolean
NodeUtils.countChildren(node: LexicalNode): number
NodeUtils.findNodesByType<T>(node: LexicalNode, type: T['type']): T[]
NodeUtils.createTextNode(text: string, format?: number): TextNode
NodeUtils.createParagraphNode(children?: LexicalNode[]): ParagraphNode
NodeUtils.createHeadingNode(tag: HeadingTag, children?: LexicalNode[]): HeadingNode
NodeUtils.createAIEmbeddingNode(content: string, isLoading?: boolean): AIEmbeddingNode
```

#### NoteUtils

Utilities for working with notes.

```tsx
NoteUtils.extractText(note: Note): string
NoteUtils.getBrief(note: Note): BriefNode[]
NoteUtils.countWords(note: Note): number
NoteUtils.countCharacters(note: Note): number
NoteUtils.findNodesByType<T>(note: Note, type: T['type']): T[]
NoteUtils.isEmpty(note: Note): boolean
NoteUtils.getNodeById(note: Note, id: number): LexicalNode | null
NoteUtils.createEmptyNote(noteId: string): Note
NoteUtils.addNode(note: Note, node: LexicalNode, position?: number): Note
NoteUtils.removeNode(note: Note, nodeId: number): Note
NoteUtils.updateNode(note: Note, nodeId: number, newNode: LexicalNode): Note
```

#### MessageUtils

Utilities for working with messages.

```tsx
MessageUtils.createMessage(content: string, role?: string): Message
MessageUtils.createUserMessage(content: string): Message
MessageUtils.createAssistantMessage(content: string): Message
MessageUtils.createSystemMessage(content: string): Message
MessageUtils.filterByRole(messages: Message[], role: string): Message[]
MessageUtils.getLastMessage(messages: Message[]): Message | null
MessageUtils.getLastMessageByRole(messages: Message[], role: string): Message | null
MessageUtils.countByRole(messages: Message[], role: string): number
MessageUtils.truncate(messages: Message[], maxCount: number): Message[]
MessageUtils.formatForDisplay(message: Message): string
```

#### ChatActionUtils

Utilities for working with chat actions.

```tsx
ChatActionUtils.isReply(action: ChatAction): boolean
ChatActionUtils.isInsertNode(action: ChatAction): boolean
ChatActionUtils.isModifyNode(action: ChatAction): boolean
ChatActionUtils.applyToNote(note: Note, action: ChatAction): Note
ChatActionUtils.getDescription(action: ChatAction): string
```

## Type Definitions

### Core Types

```tsx
interface Note {
  noteId: string;
  lexicalState: LexicalState;
}

interface Message {
  content: string;
  role: string;
}

interface ChatAction {
  action: 'reply' | 'insert_node' | 'modify_node';
  // ... specific properties based on action type
}

type AgentStatus = 'idle' | 'starting' | 'running' | 'error';

interface AgentError {
  message: string;
  code?: string;
}
```

### Configuration Types

```tsx
interface AgentConfig {
  jwt: string;
  autoStart?: boolean;
}

interface ChatConfig {
  maxMessages?: number;
  timeout?: number;
}
```

## Advanced Usage

### Custom Error Handling

```tsx
function MyApp() {
  const handleError = (error: AgentError) => {
    console.error('Agent error:', error);
    // Custom error handling logic
  };

  return (
    <AgentProvider onError={handleError}>
      <YourApp />
    </AgentProvider>
  );
}
```

### Persistent Conversations

```tsx
function ChatWithHistory() {
  const { 
    conversationHistory, 
    startNewConversation,
    loadConversation 
  } = useConversation({ 
    persistKey: 'my-app-conversations' 
  });

  return (
    <div>
      <button onClick={startNewConversation}>
        New Conversation
      </button>
      {conversationHistory.map((_, index) => (
        <button key={index} onClick={() => loadConversation(index)}>
          Load Conversation {index + 1}
        </button>
      ))}
    </div>
  );
}
```

### Working with Notes

```tsx
function NoteEditor() {
  const [note, setNote] = useState(NoteUtils.createEmptyNote('my-note'));
  const { sendMessage } = useChat();

  const handleAgentAction = async (userMessage: string) => {
    const action = await sendMessage(userMessage, 0, note);
    
    if (action && action.action !== 'reply') {
      const updatedNote = ChatActionUtils.applyToNote(note, action);
      setNote(updatedNote);
    }
  };

  return (
    <div>
      <div>Word count: {NoteUtils.countWords(note)}</div>
      <div>Character count: {NoteUtils.countCharacters(note)}</div>
      <button onClick={() => handleAgentAction('Add a paragraph')}>
        Ask Agent to Add Content
      </button>
    </div>
  );
}
```

## Development

### Building the SDK

```bash
npm run build
```

### Running Tests

```bash
npm test
```

### Development Mode

```bash
npm run dev
```

## Requirements

- React >= 16.8.0
- TypeScript >= 4.0.0
- WASM support in the browser

## License

MIT

## Contributing

Contributions are welcome! Please read the contributing guidelines before submitting PRs.

## Support

For support, please open an issue on the GitHub repository or contact the maintainers. 