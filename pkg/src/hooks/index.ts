import { useCallback, useEffect, useState } from 'react';
import { useAgentContext } from '../context/AgentContext';
import { 
  AgentConfig, 
  ChatAction, 
  Message, 
  Note, 
  AgentStatus, 
  AgentError, 
  ChatConfig 
} from '../types';

/**
 * Main hook for interacting with the agent
 * Provides access to agent state and controls
 */
export const useAgent = (config?: AgentConfig) => {
  const context = useAgentContext();
  const [isInitialized, setIsInitialized] = useState(false);
  
  // Initialize agent if config is provided
  useEffect(() => {
    if (config && !isInitialized && !context.agent) {
      context.initializeAgent(config)
        .then(() => setIsInitialized(true))
        .catch(console.error);
    }
  }, [config, isInitialized, context.agent, context.initializeAgent]);
  
  const initialize = useCallback(async (agentConfig: AgentConfig) => {
    await context.initializeAgent(agentConfig);
    setIsInitialized(true);
  }, [context.initializeAgent]);
  
  return {
    // Agent state
    agent: context.agent,
    status: context.status,
    error: context.error,
    isReady: context.isReady(),
    isInitialized,
    
    // Agent controls
    initialize,
    start: context.startAgent,
    stop: context.stopAgent,
    clearError: context.clearError,
    
    // Chat functionality
    chat: context.chat,
  };
};

/**
 * Hook for managing chat conversations with the agent
 */
export const useChat = (config?: ChatConfig) => {
  const { chat: agentChat, isReady, error } = useAgent();
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [chatError, setChatError] = useState<AgentError | null>(null);
  
  // Chat configuration
  const maxMessages = config?.maxMessages || 50;
  const timeout = config?.timeout || 30000;
  
  // Add message to conversation
  const addMessage = useCallback((message: Message) => {
    setMessages(prev => {
      const newMessages = [...prev, message];
      // Limit message history
      if (newMessages.length > maxMessages) {
        return newMessages.slice(-maxMessages);
      }
      return newMessages;
    });
  }, [maxMessages]);
  
  // Send message to agent
  const sendMessage = useCallback(async (
    content: string,
    cursorPosition: number,
    note: Note,
    role: string = 'user'
  ): Promise<ChatAction | null> => {
    if (!isReady) {
      const error = { message: 'Agent is not ready', code: 'AGENT_NOT_READY' };
      setChatError(error);
      return null;
    }
    
    try {
      setIsLoading(true);
      setChatError(null);
      
      // Add user message
      const userMessage: Message = { content, role };
      addMessage(userMessage);
      
      // Create timeout promise
      const timeoutPromise = new Promise<never>((_, reject) => {
        setTimeout(() => reject(new Error('Chat timeout')), timeout);
      });
      
      // Send to agent with timeout
      const chatPromise = agentChat([...messages, userMessage], cursorPosition, note);
      const action = await Promise.race([chatPromise, timeoutPromise]);
      
      // Add agent response if it's a reply
      if (action.action === 'reply') {
        const agentMessage: Message = { content: action.content, role: 'assistant' };
        addMessage(agentMessage);
      }
      
      return action;
    } catch (err) {
      const error = {
        message: err instanceof Error ? err.message : 'Chat failed',
        code: 'CHAT_ERROR'
      };
      setChatError(error);
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [isReady, agentChat, messages, addMessage, timeout]);
  
  // Clear conversation
  const clearMessages = useCallback(() => {
    setMessages([]);
  }, []);
  
  // Clear chat error
  const clearChatError = useCallback(() => {
    setChatError(null);
  }, []);
  
  // Get last message
  const lastMessage = messages[messages.length - 1] || null;
  
  return {
    // Chat state
    messages,
    isLoading,
    error: chatError || error,
    lastMessage,
    
    // Chat controls
    sendMessage,
    addMessage,
    clearMessages,
    clearError: clearChatError,
  };
};

/**
 * Hook for monitoring agent status
 */
export const useAgentStatus = () => {
  const { status, error, isReady } = useAgent();
  const [statusHistory, setStatusHistory] = useState<AgentStatus[]>([]);
  
  // Track status changes
  useEffect(() => {
    setStatusHistory(prev => [...prev, status].slice(-10)); // Keep last 10 statuses
  }, [status]);
  
  return {
    status,
    error,
    isReady,
    statusHistory,
    
    // Status helpers
    isIdle: status === 'idle',
    isStarting: status === 'starting',
    isRunning: status === 'running',
    hasError: status === 'error',
  };
};

/**
 * Hook for creating and managing messages
 */
export const useMessages = (initialMessages: Message[] = []) => {
  const [messages, setMessages] = useState<Message[]>(initialMessages);
  
  const addMessage = useCallback((message: Message) => {
    setMessages(prev => [...prev, message]);
  }, []);
  
  const removeMessage = useCallback((index: number) => {
    setMessages(prev => prev.filter((_, i) => i !== index));
  }, []);
  
  const updateMessage = useCallback((index: number, message: Message) => {
    setMessages(prev => prev.map((msg, i) => i === index ? message : msg));
  }, []);
  
  const clearMessages = useCallback(() => {
    setMessages([]);
  }, []);
  
  const getMessagesByRole = useCallback((role: string) => {
    return messages.filter(msg => msg.role === role);
  }, [messages]);
  
  return {
    messages,
    addMessage,
    removeMessage,
    updateMessage,
    clearMessages,
    getMessagesByRole,
    
    // Message helpers
    userMessages: getMessagesByRole('user'),
    assistantMessages: getMessagesByRole('assistant'),
    systemMessages: getMessagesByRole('system'),
    isEmpty: messages.length === 0,
    count: messages.length,
  };
};

/**
 * Hook for handling agent errors
 */
export const useAgentError = () => {
  const { error, clearError } = useAgent();
  const [errorHistory, setErrorHistory] = useState<AgentError[]>([]);
  
  // Track error history
  useEffect(() => {
    if (error) {
      setErrorHistory(prev => [...prev, error].slice(-5)); // Keep last 5 errors
    }
  }, [error]);
  
  const clearErrorHistory = useCallback(() => {
    setErrorHistory([]);
  }, []);
  
  return {
    error,
    errorHistory,
    clearError,
    clearErrorHistory,
    
    // Error helpers
    hasError: error !== null,
    isWasmError: error?.code === 'WASM_LOAD_ERROR',
    isAgentError: error?.code === 'AGENT_INIT_ERROR' || error?.code === 'AGENT_START_ERROR',
    isChatError: error?.code === 'CHAT_ERROR',
  };
};

/**
 * Hook for advanced chat management with conversation history
 */
export const useConversation = (config?: ChatConfig & { persistKey?: string }) => {
  const { sendMessage, ...chatHook } = useChat(config);
  const [conversationHistory, setConversationHistory] = useState<Message[][]>([]);
  
  // Persist conversation if key is provided
  useEffect(() => {
    if (config?.persistKey) {
      const saved = localStorage.getItem(`conversation_${config.persistKey}`);
      if (saved) {
        try {
          const parsed = JSON.parse(saved);
          setConversationHistory(parsed);
        } catch (e) {
          console.warn('Failed to parse saved conversation:', e);
        }
      }
    }
  }, [config?.persistKey]);
  
  // Save conversation on changes
  useEffect(() => {
    if (config?.persistKey && conversationHistory.length > 0) {
      localStorage.setItem(
        `conversation_${config.persistKey}`,
        JSON.stringify(conversationHistory)
      );
    }
  }, [conversationHistory, config?.persistKey]);
  
  const startNewConversation = useCallback(() => {
    if (chatHook.messages.length > 0) {
      setConversationHistory(prev => [...prev, chatHook.messages]);
      chatHook.clearMessages();
    }
  }, [chatHook.messages, chatHook.clearMessages]);
  
  const loadConversation = useCallback((index: number) => {
    if (index >= 0 && index < conversationHistory.length) {
      chatHook.clearMessages();
      const conversation = conversationHistory[index];
      if (conversation) {
        conversation.forEach(msg => chatHook.addMessage(msg));
      }
    }
  }, [conversationHistory, chatHook.clearMessages, chatHook.addMessage]);
  
  const deleteConversation = useCallback((index: number) => {
    setConversationHistory(prev => prev.filter((_, i) => i !== index));
  }, []);
  
  const clearAllConversations = useCallback(() => {
    setConversationHistory([]);
    if (config?.persistKey) {
      localStorage.removeItem(`conversation_${config.persistKey}`);
    }
  }, [config?.persistKey]);
  
  return {
    ...chatHook,
    sendMessage,
    
    // Conversation management
    conversationHistory,
    startNewConversation,
    loadConversation,
    deleteConversation,
    clearAllConversations,
    
    // Helpers
    hasHistory: conversationHistory.length > 0,
    conversationCount: conversationHistory.length,
  };
}; 