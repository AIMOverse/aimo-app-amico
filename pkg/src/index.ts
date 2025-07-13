/**
 * @aimoverse/note-agent-react
 * 
 * React TypeScript SDK for WASM note agent
 * 
 * This library provides React hooks and components for interacting with
 * the AIMO note agent WASM module, including chat functionality, note
 * manipulation, and real-time agent interactions.
 */

// Export all types
export * from './types';

// Export context and provider
export { 
  AgentProvider, 
  useAgentContext,
  type AgentContextValue,
  type AgentProviderProps
} from './context/AgentContext';

// Export hooks
export {
  useAgent,
  useChat,
  useAgentStatus,
  useMessages,
  useAgentError,
  useConversation
} from './hooks';

// Export utilities
export {
  NodeUtils,
  NoteUtils,
  MessageUtils,
  ChatActionUtils,
  FormatUtils,
  DebugUtils
} from './utils';

// Import for default export
import { AgentProvider, useAgentContext } from './context/AgentContext';
import {
  useAgent,
  useChat,
  useAgentStatus,
  useMessages,
  useAgentError,
  useConversation
} from './hooks';
import {
  NodeUtils,
  NoteUtils,
  MessageUtils,
  ChatActionUtils,
  FormatUtils,
  DebugUtils
} from './utils';

// Version information
export const VERSION = '0.1.0';

// Default export for convenience
export default {
  AgentProvider,
  useAgent,
  useChat,
  useAgentStatus,
  NodeUtils,
  NoteUtils,
  MessageUtils,
  ChatActionUtils,
  FormatUtils,
  DebugUtils,
  VERSION,
}; 