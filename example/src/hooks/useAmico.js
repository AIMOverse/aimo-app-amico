import { useState, useEffect, useCallback } from "react";

/**
 * Hook to initialize and manage the WASM module
 * @returns {Object} { isReady, error, wasmModule }
 */
export function useAmico() {
  const [isReady, setIsReady] = useState(false);
  const [error, setError] = useState(null);
  const [wasmModule, setWasmModule] = useState(null);

  useEffect(() => {
    const initWasm = async () => {
      try {
        setError(null);

        // Dynamic import of WASM module and binary
        const [wasmModule, wasmUrl] = await Promise.all([
          import("@aimoverse/aimo-app-amico"),
          import("@aimoverse/aimo-app-amico/aimo_app_amico_bg.wasm?url"),
        ]);

        console.log("WASM module loaded:", wasmModule);
        console.log("WASM binary URL:", wasmUrl.default);

        // Initialize the WASM module with the proper binary URL
        await wasmModule.default(wasmUrl.default);
        console.log("WASM module initialized");

        setWasmModule(wasmModule);
        setIsReady(true);
      } catch (err) {
        console.error("Failed to initialize WASM:", err);
        setError(err);
      }
    };

    initWasm();
  }, []);

  return { isReady, error, wasmModule };
}

/**
 * Hook to manage the agent runtime
 * @param {string} token - JWT token for authentication
 * @returns {Object} { agent, isRunning, startAgent, sendMessage, loading, messages }
 */
export function useAgent(token) {
  const { isReady, error: wasmError, wasmModule } = useAmico();
  const [agent, setAgent] = useState(null);
  const [isRunning, setIsRunning] = useState(false);
  const [loading, setLoading] = useState(false);
  const [messages, setMessages] = useState([]);
  const [error, setError] = useState(null);

  const startAgent = useCallback(() => {
    if (!wasmModule || !token) {
      setError("WASM not ready or token missing");
      return;
    }

    try {
      const runtime = new wasmModule.AgentWasmRuntime(token);
      console.log("AgentWasmRuntime created:", runtime);

      runtime.start();
      console.log("Agent started successfully");

      setAgent(runtime);
      setIsRunning(runtime.is_running());

      // Add system message
      setMessages([
        {
          id: Date.now(),
          sender: "system",
          content: "Agent is now running and ready to chat!",
          timestamp: new Date().toLocaleTimeString(),
        },
      ]);

      setError(null);
    } catch (err) {
      console.error("Error starting agent:", err);
      setError(err);
    }
  }, [wasmModule, token]);

  const sendMessage = useCallback(
    async (content) => {
      if (!agent || !content.trim() || loading) return;

      const userMessage = {
        id: Date.now(),
        sender: "user",
        content: content.trim(),
        timestamp: new Date().toLocaleTimeString(),
      };

      setMessages((prev) => [...prev, userMessage]);
      setLoading(true);

      try {
        const message = new wasmModule.Message(userMessage.content, "user");
        const response = await agent.chat([message]);

        const agentMessage = {
          id: Date.now() + 1,
          sender: "agent",
          content: response,
          timestamp: new Date().toLocaleTimeString(),
        };

        setMessages((prev) => [...prev, agentMessage]);
        setError(null);
      } catch (err) {
        console.error("Error sending chat:", err);
        const errorMessage = {
          id: Date.now() + 1,
          sender: "system",
          content: `Error: ${err.toString()}`,
          timestamp: new Date().toLocaleTimeString(),
        };
        setMessages((prev) => [...prev, errorMessage]);
        setError(err);
      } finally {
        setLoading(false);
      }
    },
    [agent, wasmModule, loading]
  );

  return {
    // Status
    isReady,
    isRunning,
    loading,
    error: error || wasmError,

    // Data
    messages,

    // Actions
    startAgent,
    sendMessage,
  };
}
