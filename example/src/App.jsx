import { useState } from "react";
import "./App.css";
import Login from "./login";
import { useAgent } from "./hooks/useAmico";

function App() {
  const [inputMessage, setInputMessage] = useState("");

  // Get token from localStorage
  const token = localStorage.getItem("access_token");

  // Use the clean hooks API
  const {
    isReady,
    isRunning,
    loading,
    error,
    messages,
    startAgent,
    sendMessage,
  } = useAgent(token);

  const handleSendMessage = async () => {
    if (!inputMessage.trim() || loading) return;

    await sendMessage(inputMessage);
    setInputMessage("");
  };

  const handleKeyPress = (e) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const handleStartAgent = () => {
    if (!token || token === "") {
      alert("No token found, please login first");
      return;
    }
    startAgent();
  };

  return (
    <>
      <h1>Amico WASM Chat Agent</h1>

      <Login />

      <div className="card">
        {error && (
          <div style={{ color: "red", marginBottom: "10px" }}>
            Error: {error.toString()}
          </div>
        )}

        {!isRunning ? (
          <div style={{ marginBottom: "20px" }}>
            <button
              onClick={handleStartAgent}
              disabled={!isReady}
              style={{
                backgroundColor: isReady ? "#646cff" : "#888",
                padding: "10px 20px",
                borderRadius: "8px",
                color: "white",
                cursor: isReady ? "pointer" : "not-allowed",
                fontSize: "16px",
              }}
            >
              {isReady ? "Start Agent" : "Loading WASM..."}
            </button>
            <p>
              {isReady
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
              {messages.map((msg) => (
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
              Agent Status: {isRunning ? "🟢 Running" : "🔴 Stopped"}
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
