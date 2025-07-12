# Lexical Node Schema Documentation

> **Author**: Archer
> **Last Edit**: 2025-07-12

## Table of Contents

1. [Overview](#overview)
2. [Core Node Structure](#core-node-structure)
3. [Standard Node Types](#standard-node-types)
4. [Custom Node Types](#custom-node-types)
5. [Node Properties](#node-properties)
6. [Document Structure](#document-structure)
7. [Implementation Details](#implementation-details)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Overview

This document describes the Lexical node schema used in the AIMO Native App. The schema defines how content is structured, stored, and manipulated within the Lexical editor framework. All content is represented as a tree of nodes, each with specific properties and behaviors.

## Core Node Structure

Every Lexical node follows a base structure:

```typescript
interface BaseNode {
  type: string;           // Node type identifier
  version: number;        // Schema version for compatibility
  children?: Node[];      // Child nodes (for container nodes)
  direction?: "ltr" | "rtl" | null;  // Text direction
  format?: string | number;          // Format flags
  indent?: number;        // Indentation level
}
```

## Standard Node Types

### Root Node

The root node is the top-level container for the entire document:

```json
{
  "type": "root",
  "children": [...],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Text Node

The most basic node type for text content:

```json
{
  "type": "text",
  "text": "Hello World",
  "format": 1,  // Binary flags: 1=bold, 2=italic, 4=underline, 8=strikethrough
  "version": 1
}
```

Format flags can be combined (e.g., `3` = bold + italic).

### Paragraph Node

Container for text and inline elements:

```json
{
  "type": "paragraph",
  "children": [
    {
      "type": "text",
      "text": "This is a paragraph",
      "format": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Heading Node

For document headings (h1-h6):

```json
{
  "type": "heading",
  "tag": "h1",
  "children": [
    {
      "type": "text",
      "text": "Main Heading",
      "format": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### List Node

Container for ordered and unordered lists:

```json
{
  "type": "list",
  "listType": "bullet",  // "bullet" or "number"
  "start": 1,
  "children": [
    {
      "type": "listitem",
      "children": [...],
      "direction": null,
      "format": "",
      "indent": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### List Item Node

Individual items within lists:

```json
{
  "type": "listitem",
  "children": [
    {
      "type": "text",
      "text": "List item content",
      "format": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Quote Node

For blockquotes:

```json
{
  "type": "quote",
  "children": [
    {
      "type": "paragraph",
      "children": [
        {
          "type": "text",
          "text": "Quoted text",
          "format": 0,
          "version": 1
        }
      ],
      "direction": null,
      "format": "",
      "indent": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Code Node

For inline code:

```json
{
  "type": "code",
  "text": "console.log('Hello')",
  "format": 0,
  "version": 1
}
```

### Code Block Node

For code blocks with syntax highlighting:

```json
{
  "type": "code",
  "language": "javascript",
  "children": [
    {
      "type": "text",
      "text": "function hello() {\n  console.log('Hello World');\n}",
      "format": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Link Node

For hyperlinks:

```json
{
  "type": "link",
  "url": "https://example.com",
  "children": [
    {
      "type": "text",
      "text": "Link text",
      "format": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Auto Link Node

For automatically detected links:

```json
{
  "type": "autolink",
  "url": "https://example.com",
  "children": [
    {
      "type": "text",
      "text": "https://example.com",
      "format": 0,
      "version": 1
    }
  ],
  "direction": null,
  "format": "",
  "indent": 0,
  "version": 1
}
```

### Hashtag Node

For hashtags:

```json
{
  "type": "hashtag",
  "text": "#example",
  "format": 0,
  "version": 1
}
```

### Table Node

For tables:

```json
{
  "type": "table",
  "children": [
    {
      "type": "tablerow",
      "children": [
        {
          "type": "tablecell",
          "children": [...],
          "headerState": 0,
          "colSpan": 1,
          "rowSpan": 1,
          "version": 1
        }
      ],
      "version": 1
    }
  ],
  "version": 1
}
```

### Page Break Node

For page breaks:

```json
{
  "type": "page-break",
  "version": 1
}
```

## Custom Node Types

The application defines several custom node types for specialized functionality:

### AI Embedding Node

For AI-generated content:

```typescript
interface SerializedAiEmbeddingNode {
  type: "ai-embedding";
  content: string;        // AI generated content
  isLoading: boolean;     // Whether AI is still generating
  version: 1;
}
```

**Example:**
```json
{
  "type": "ai-embedding",
  "content": "AI generated response about the topic...",
  "isLoading": false,
  "version": 1
}
```

### Voice Input Node

For voice-to-text transcriptions:

```typescript
interface SerializedVoiceInputNode {
  type: "voice-input";
  content: string;        // Transcribed voice content
  version: 1;
}
```

**Example:**
```json
{
  "type": "voice-input",
  "content": "This text was transcribed from voice input",
  "version": 1
}
```

### Chat Message Node

For individual chat messages:

```typescript
interface SerializedChatMessageNode {
  type: "chat-message";
  sender: "user" | "agent" | "system";
  content: string;        // Message content
  timestamp: string;      // ISO timestamp
  version: 1;
}
```

**Example:**
```json
{
  "type": "chat-message",
  "sender": "user",
  "content": "Hello, can you help me with this?",
  "timestamp": "2023-12-01T10:00:00.000Z",
  "version": 1
}
```

### Chat Session Node

For chat sessions containing multiple messages:

```typescript
interface SerializedChatSessionNode {
  type: "chat-session";
  sessionId: string;      // Unique session identifier
  isActive: boolean;      // Whether session is active
  messages: Message[];    // Array of chat messages
  version: 1;
}
```

**Example:**
```json
{
  "type": "chat-session",
  "sessionId": "session-123",
  "isActive": true,
  "messages": [
    {
      "id": 1,
      "sender": "user",
      "content": "Hello",
      "timestamp": "2023-12-01T10:00:00.000Z"
    },
    {
      "id": 2,
      "sender": "agent",
      "content": "Hi! How can I help you?",
      "timestamp": "2023-12-01T10:01:00.000Z"
    }
  ],
  "version": 1
}
```

### Mention Node

For @mentions:

```typescript
interface SerializedMentionNode {
  type: "mention";
  mentionName: string;    // The mentioned entity
  text: string;           // Display text
  format: number;         // Text formatting
  version: 1;
}
```

**Example:**
```json
{
  "type": "mention",
  "mentionName": "@john",
  "text": "@john",
  "format": 0,
  "version": 1
}
```

## Node Properties

### Common Properties

All nodes share these base properties:

- **type**: String identifier for the node type
- **version**: Schema version number for compatibility
- **children**: Array of child nodes (for container nodes)
- **direction**: Text direction (`"ltr"`, `"rtl"`, or `null`)
- **format**: Format flags or styling information
- **indent**: Indentation level (number)

### Text-Specific Properties

Text nodes have additional properties:

- **text**: The actual text content
- **format**: Binary flags for styling (bold, italic, underline, strikethrough)

### List-Specific Properties

List nodes have:

- **listType**: `"bullet"` or `"number"`
- **start**: Starting number for ordered lists

### Link-Specific Properties

Link nodes have:

- **url**: The target URL
- **rel**: Link relationship (optional)
- **target**: Link target (optional)

### Table-Specific Properties

Table cells have:

- **headerState**: Whether it's a header cell
- **colSpan**: Number of columns spanned
- **rowSpan**: Number of rows spanned

## Document Structure

### Empty Document

```json
{
  "root": {
    "children": [
      {
        "children": [],
        "direction": null,
        "format": "",
        "indent": 0,
        "type": "paragraph",
        "version": 1
      }
    ],
    "direction": null,
    "format": "",
    "indent": 0,
    "type": "root",
    "version": 1
  }
}
```

### Complex Document Example

```json
{
  "root": {
    "children": [
      {
        "type": "heading",
        "tag": "h1",
        "children": [
          {
            "type": "text",
            "text": "Document Title",
            "format": 0,
            "version": 1
          }
        ],
        "direction": null,
        "format": "",
        "indent": 0,
        "version": 1
      },
      {
        "type": "paragraph",
        "children": [
          {
            "type": "text",
            "text": "This is a paragraph with ",
            "format": 0,
            "version": 1
          },
          {
            "type": "text",
            "text": "bold text",
            "format": 1,
            "version": 1
          },
          {
            "type": "text",
            "text": " and ",
            "format": 0,
            "version": 1
          },
          {
            "type": "link",
            "url": "https://example.com",
            "children": [
              {
                "type": "text",
                "text": "a link",
                "format": 0,
                "version": 1
              }
            ],
            "direction": null,
            "format": "",
            "indent": 0,
            "version": 1
          }
        ],
        "direction": null,
        "format": "",
        "indent": 0,
        "version": 1
      },
      {
        "type": "ai-embedding",
        "content": "This is AI-generated content that was inserted into the document.",
        "isLoading": false,
        "version": 1
      }
    ],
    "direction": null,
    "format": "",
    "indent": 0,
    "type": "root",
    "version": 1
  }
}
```

## Implementation Details

### Database Storage

The schema is stored in IndexedDB via Dexie:

```typescript
export interface LexicalNoteContent {
  id?: string;
  noteId: string;
  lexicalState: any;      // The full Lexical state
  lastLexicalUpdate: Date;
  plainText?: string;     // Extracted for search
  lastSavedAt: Date;
  aiEmbeddings?: LexicalAIEmbedding[];
}
```

### Node Registration

Custom nodes are registered in the editor configuration:

```typescript
const editorConfig = {
  nodes: [
    HeadingNode,
    ListNode,
    ListItemNode,
    HashtagNode,
    QuoteNode,
    CodeNode,
    CodeHighlightNode,
    TableNode,
    TableCellNode,
    TableRowNode,
    AutoLinkNode,
    LinkNode,
    PageBreakNode,
    AiEmbeddingNode,          // Custom
    VoiceInputNode,           // Custom
    MentionNode,              // Custom
    ChatMessageNode,          // Custom
    ChatSessionNode,          // Custom
  ],
}
```

### Serialization/Deserialization

Nodes implement serialization methods:

```typescript
export class AiEmbeddingNode extends DecoratorNode<JSX.Element> {
  exportJSON(): SerializedAiEmbeddingNode {
    return {
      content: this.__content,
      isLoading: this.__isLoading,
      type: "ai-embedding",
      version: 1,
    };
  }

  static importJSON(serializedNode: SerializedAiEmbeddingNode): AiEmbeddingNode {
    const { content = "", isLoading = false } = serializedNode;
    return new AiEmbeddingNode(content, isLoading);
  }
}
```

## Best Practices

### 1. Version Management

Always include version numbers in node schemas to handle future changes:

```typescript
interface SerializedCustomNode {
  type: "custom-node";
  version: 1;  // Always include version
  // ... other properties
}
```

### 2. Validation

Validate node data when importing:

```typescript
static importJSON(serializedNode: SerializedCustomNode): CustomNode {
  const { content = "", version = 1 } = serializedNode;
  
  // Validate required fields
  if (!content) {
    throw new Error("Content is required for CustomNode");
  }
  
  return new CustomNode(content);
}
```

### 3. Backward Compatibility

Handle version differences gracefully:

```typescript
static importJSON(serializedNode: SerializedCustomNode): CustomNode {
  const { version = 1 } = serializedNode;
  
  switch (version) {
    case 1:
      return importV1(serializedNode);
    case 2:
      return importV2(serializedNode);
    default:
      throw new Error(`Unsupported version: ${version}`);
  }
}
```

### 4. Type Safety

Use TypeScript interfaces for all node types:

```typescript
export interface SerializedCustomNode extends SerializedLexicalNode {
  type: "custom-node";
  customProperty: string;
  version: 1;
}
```

### 5. Testing

Test serialization/deserialization:

```typescript
describe("CustomNode", () => {
  it("should serialize and deserialize correctly", () => {
    const node = new CustomNode("test content");
    const serialized = node.exportJSON();
    const deserialized = CustomNode.importJSON(serialized);
    
    expect(deserialized.getContent()).toBe("test content");
  });
});
```

## Examples

### Creating a Custom Node

```typescript
import { DecoratorNode, NodeKey, SerializedLexicalNode } from "lexical";

export interface SerializedCustomNode extends SerializedLexicalNode {
  type: "custom-node";
  content: string;
  version: 1;
}

export class CustomNode extends DecoratorNode<JSX.Element> {
  __content: string;

  static getType(): string {
    return "custom-node";
  }

  constructor(content: string, key?: NodeKey) {
    super(key);
    this.__content = content;
  }

  exportJSON(): SerializedCustomNode {
    return {
      type: "custom-node",
      content: this.__content,
      version: 1,
    };
  }

  static importJSON(serializedNode: SerializedCustomNode): CustomNode {
    const { content } = serializedNode;
    return new CustomNode(content);
  }

  decorate(): JSX.Element {
    return <div>{this.__content}</div>;
  }
}
```

### Working with the Document

```typescript
// Reading the document
const lexicalState = await lexicalDb.getLexicalState(noteId);

// Extracting plain text
function extractPlainText(lexicalState: any): string {
  const extractFromNode = (node: any): string => {
    if (node.text) return node.text;
    if (node.children) {
      return node.children.map(extractFromNode).join(" ");
    }
    return "";
  };
  
  return extractFromNode(lexicalState.root);
}

// Updating the document
await lexicalDb.updateLexicalState(noteId, newLexicalState);
```

This schema provides a flexible, extensible foundation for rich text editing while maintaining type safety and backward compatibility. 