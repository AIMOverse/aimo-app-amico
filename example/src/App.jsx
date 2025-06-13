import { useState, useEffect } from "react";
import "./App.css";
import Login from "./login";

function App() {
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const [wasmModule, setWasmModule] = useState(null);
  const [agentRuntime, setAgentRuntime] = useState(null);
  const [agentRunning, setAgentRunning] = useState(false);
  const [chatMessages, setChatMessages] = useState([]);
  const [inputMessage, setInputMessage] = useState("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const initWasm = async () => {
      try {
        // Dynamic import of WASM module
        const wasm = await import("amico-wasm");
        console.log("WASM module loaded:", wasm);
        setWasmModule(wasm);
        setWasmLoaded(true);
      } catch (error) {
        console.error("Failed to initialize WASM:", error);
      }
    };

    initWasm();
  }, []);

  const handleStartAgent = () => {
    if (!wasmModule) return;

    try {
      const token = localStorage.getItem("access_token");

      if (!token) {
        alert("No token found, please login first");
        return;
      }

      const runtime = new wasmModule.AgentWasmRuntime(token);
      console.log("AgentWasmRuntime created:", runtime);

      runtime.start();
      console.log("Agent started successfully");

      setAgentRuntime(runtime);
      setAgentRunning(runtime.is_running());

      // Add a system message to indicate the agent is ready
      setChatMessages([
        {
          id: Date.now(),
          sender: "system",
          content: "Agent is now running and ready to chat!",
          timestamp: new Date().toLocaleTimeString(),
        },
      ]);
    } catch (error) {
      console.error("Error starting agent:", error);
    }
  };

  const handleSendMessage = async () => {
    if (!agentRuntime || !inputMessage.trim() || loading) return;

    const userMessage = {
      id: Date.now(),
      sender: "user",
      content: inputMessage.trim(),
      timestamp: new Date().toLocaleTimeString(),
    };

    // Add user message to chat
    setChatMessages((prev) => [...prev, userMessage]);
    setInputMessage("");
    setLoading(true);

    try {
      // Create a Message object with content and role
      const message = new wasmModule.Message(userMessage.content, "user");

      // Send messages array to agent and get response
      const response = await agentRuntime.chat([message]);

      // Add agent response to chat
      const agentMessage = {
        id: Date.now() + 1,
        sender: "agent",
        content: response,
        timestamp: new Date().toLocaleTimeString(),
      };

      setChatMessages((prev) => [...prev, agentMessage]);
    } catch (error) {
      console.error("Error sending chat:", error);

      // Add error message to chat
      const errorMessage = {
        id: Date.now() + 1,
        sender: "system",
        content: `Error: ${error.toString()}`,
        timestamp: new Date().toLocaleTimeString(),
      };

      setChatMessages((prev) => [...prev, errorMessage]);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <>
      <h1>Amico WASM Chat Agent</h1>

      <Login />

      <div className="card">
        {!agentRunning ? (
          <div style={{ marginBottom: "20px" }}>
            <button
              onClick={handleStartAgent}
              disabled={!wasmLoaded}
              style={{
                backgroundColor: wasmLoaded ? "#646cff" : "#888",
                padding: "10px 20px",
                borderRadius: "8px",
                color: "white",
                cursor: wasmLoaded ? "pointer" : "not-allowed",
                fontSize: "16px",
              }}
            >
              {wasmLoaded ? "Start Agent" : "Loading WASM..."}
            </button>
            <p>
              {wasmLoaded
                ? "WASM loaded successfully! Click to start the chat agent."
                : "Loading WASM module..."}
            </p>
          </div>
        ) : (
          <div>
            {/* Chat Messages */}
            <div
              style={{
                border: "1px solid #ccc",
                borderRadius: "8px",
                height: "400px",
                overflowY: "auto",
                padding: "10px",
                marginBottom: "10px",
                backgroundColor: "#f9f9f9",
              }}
            >
              {chatMessages.map((msg) => (
                <div
                  key={msg.id}
                  style={{
                    marginBottom: "10px",
                    padding: "8px",
                    borderRadius: "4px",
                    backgroundColor:
                      msg.sender === "user"
                        ? "#e3f2fd"
                        : msg.sender === "agent"
                        ? "#e8f5e8"
                        : "#fff3e0",
                  }}
                >
                  <div
                    style={{
                      fontSize: "12px",
                      color: "#666",
                      marginBottom: "4px",
                    }}
                  >
                    {msg.sender === "user"
                      ? "👤 You"
                      : msg.sender === "agent"
                      ? "🤖 Agent"
                      : "⚙️ System"}{" "}
                    • {msg.timestamp}
                  </div>
                  <div>{msg.content}</div>
                </div>
              ))}
              {loading && (
                <div
                  style={{
                    padding: "8px",
                    fontStyle: "italic",
                    color: "#666",
                  }}
                >
                  🤖 Agent is thinking...
                </div>
              )}
            </div>

            {/* Chat Input */}
            <div style={{ display: "flex", gap: "10px" }}>
              <input
                type="text"
                value={inputMessage}
                onChange={(e) => setInputMessage(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="Type your message here..."
                disabled={loading}
                style={{
                  flex: 1,
                  padding: "10px",
                  borderRadius: "4px",
                  border: "1px solid #ccc",
                  fontSize: "14px",
                }}
              />
              <button
                onClick={handleSendMessage}
                disabled={!inputMessage.trim() || loading}
                style={{
                  backgroundColor:
                    !inputMessage.trim() || loading ? "#888" : "#4CAF50",
                  color: "white",
                  border: "none",
                  padding: "10px 20px",
                  borderRadius: "4px",
                  cursor:
                    !inputMessage.trim() || loading ? "not-allowed" : "pointer",
                  fontSize: "14px",
                }}
              >
                Send
              </button>
            </div>

            <p style={{ marginTop: "20px", fontSize: "12px", color: "#666" }}>
              Agent Status:{" "}
              {agentRunning && agentRuntime?.is_running()
                ? "🟢 Running"
                : "🔴 Stopped"}
            </p>
          </div>
        )}
      </div>

      <p className="read-the-docs">
        Chat with the Amico WASM agent using the interface above
      </p>
    </>
  );
}

export default App;
