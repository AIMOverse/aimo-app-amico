import React, { createContext, useContext, useCallback, useEffect, useRef, useState } from 'react';
import { AgentWasmRuntime, AgentStatus, AgentError, AgentConfig, ChatAction, Message, Note } from '../types';

// WASM module import - this will be the actual WASM module
declare const WasmModule: {
  AgentWasmRuntime: new (jwt: string) => AgentWasmRuntime;
  start: () => void;
};

export interface AgentContextValue {
  // Agent state
  agent: AgentWasmRuntime | null;
  status: AgentStatus;
  error: AgentError | null;
  
  // Agent controls
  initializeAgent: (config: AgentConfig) => Promise<void>;
  startAgent: () => Promise<void>;
  stopAgent: () => void;
  
  // Chat functionality
  chat: (messages: Message[], cursorPosition: number, note: Note) => Promise<ChatAction>;
  
  // Utility functions
  isReady: () => boolean;
  clearError: () => void;
}

const AgentContext = createContext<AgentContextValue | undefined>(undefined);

export interface AgentProviderProps {
  children: React.ReactNode;
  /**
   * Configuration for the agent
   */
  config?: AgentConfig;
  /**
   * Custom error handler
   */
  onError?: (error: AgentError) => void;
  /**
   * Custom status change handler
   */
  onStatusChange?: (status: AgentStatus) => void;
}

export const AgentProvider: React.FC<AgentProviderProps> = ({
  children,
  config,
  onError,
  onStatusChange
}) => {
  const [agent, setAgent] = useState<AgentWasmRuntime | null>(null);
  const [status, setStatus] = useState<AgentStatus>('idle');
  const [error, setError] = useState<AgentError | null>(null);
  const [isWasmLoaded, setIsWasmLoaded] = useState(false);
  
  // Keep track of initialization promise to prevent multiple initializations
  const initializationPromise = useRef<Promise<void> | null>(null);
  
  // Update status with callback
  const updateStatus = useCallback((newStatus: AgentStatus) => {
    setStatus(newStatus);
    onStatusChange?.(newStatus);
  }, [onStatusChange]);
  
  // Handle errors
  const handleError = useCallback((err: AgentError) => {
    setError(err);
    updateStatus('error');
    onError?.(err);
  }, [onError, updateStatus]);
  
  // Clear error state
  const clearError = useCallback(() => {
    setError(null);
    if (status === 'error') {
      updateStatus('idle');
    }
  }, [status, updateStatus]);
  
  // Load WASM module
  useEffect(() => {
    const loadWasm = async () => {
      try {
        // Initialize the WASM module
        if (typeof WasmModule !== 'undefined' && WasmModule.start) {
          WasmModule.start();
          setIsWasmLoaded(true);
        } else {
          // Fallback: try to import the WASM module
          try {
            // Use dynamic import with type assertion to avoid TypeScript compilation errors
            const wasmModulePath = '../wasm/aimo_note_agent';
            const wasmModule = await import(wasmModulePath as any);
            wasmModule.default();
            setIsWasmLoaded(true);
          } catch (importErr) {
            // If WASM module doesn't exist, mark as loaded but note the issue
            console.warn('WASM module not found, proceeding without it. Make sure to build and place the WASM module in the correct location.');
            setIsWasmLoaded(true);
          }
        }
      } catch (err) {
        handleError({
          message: 'Failed to load WASM module',
          code: 'WASM_LOAD_ERROR'
        });
      }
    };
    
    loadWasm();
  }, [handleError]);
  
  // Initialize agent
  const initializeAgent = useCallback(async (agentConfig: AgentConfig) => {
    if (!isWasmLoaded) {
      throw new Error('WASM module is not loaded yet');
    }
    
    if (initializationPromise.current) {
      return initializationPromise.current;
    }
    
    initializationPromise.current = (async () => {
      try {
        clearError();
        updateStatus('starting');
        
        // Create new agent instance
        const newAgent = new WasmModule.AgentWasmRuntime(agentConfig.jwt);
        setAgent(newAgent);
        
        // Auto-start if configured
        if (agentConfig.autoStart !== false) {
          newAgent.start();
          updateStatus('running');
        } else {
          updateStatus('idle');
        }
        
      } catch (err) {
        handleError({
          message: err instanceof Error ? err.message : 'Failed to initialize agent',
          code: 'AGENT_INIT_ERROR'
        });
        throw err;
      }
    })();
    
    return initializationPromise.current;
  }, [isWasmLoaded, clearError, updateStatus, handleError]);
  
  // Start agent
  const startAgent = useCallback(async () => {
    if (!agent) {
      throw new Error('Agent is not initialized');
    }
    
    try {
      clearError();
      updateStatus('starting');
      
      agent.start();
      updateStatus('running');
    } catch (err) {
      handleError({
        message: err instanceof Error ? err.message : 'Failed to start agent',
        code: 'AGENT_START_ERROR'
      });
      throw err;
    }
  }, [agent, clearError, updateStatus, handleError]);
  
  // Stop agent
  const stopAgent = useCallback(() => {
    if (agent) {
      // Note: The WASM runtime doesn't have a stop method, so we just reset the state
      setAgent(null);
      updateStatus('idle');
      initializationPromise.current = null;
    }
  }, [agent, updateStatus]);
  
  // Chat with agent
  const chat = useCallback(async (messages: Message[], cursorPosition: number, note: Note): Promise<ChatAction> => {
    if (!agent) {
      throw new Error('Agent is not initialized');
    }
    
    if (!agent.isRunning()) {
      throw new Error('Agent is not running');
    }
    
    try {
      clearError();
      return await agent.chat(messages, cursorPosition, note);
    } catch (err) {
      const error = {
        message: err instanceof Error ? err.message : 'Chat failed',
        code: 'CHAT_ERROR'
      };
      handleError(error);
      throw error;
    }
  }, [agent, clearError, handleError]);
  
  // Check if agent is ready
  const isReady = useCallback((): boolean => {
    return agent !== null && agent.isRunning();
  }, [agent]);
  
  // Auto-initialize if config is provided
  useEffect(() => {
    if (config && isWasmLoaded && !agent && !initializationPromise.current) {
      initializeAgent(config).catch(console.error);
    }
  }, [config, isWasmLoaded, agent, initializeAgent]);
  
  const value: AgentContextValue = {
    agent,
    status,
    error,
    initializeAgent,
    startAgent,
    stopAgent,
    chat,
    isReady,
    clearError
  };
  
  return (
    <AgentContext.Provider value={value}>
      {children}
    </AgentContext.Provider>
  );
};

export const useAgentContext = (): AgentContextValue => {
  const context = useContext(AgentContext);
  if (context === undefined) {
    throw new Error('useAgentContext must be used within an AgentProvider');
  }
  return context;
}; 