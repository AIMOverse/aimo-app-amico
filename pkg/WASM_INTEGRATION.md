# WASM Integration Guide

This guide explains how to integrate your actual WASM module with the React TypeScript SDK.

## Building the WASM Module

1. **Build your Rust project to WASM:**
   ```bash
   # From the root directory
   wasm-pack build --target web --out-dir pkg/src/wasm
   ```

2. **Alternative build with specific output:**
   ```bash
   wasm-pack build --target bundler --out-dir pkg/src/wasm --out-name aimo_note_agent
   ```

## Expected WASM Module Structure

The SDK expects the WASM module to be available at `pkg/src/wasm/aimo_note_agent` with the following exports:

### Required Exports

```typescript
// The WASM module should export these at minimum:
interface WasmModule {
  AgentWasmRuntime: new (jwt: string) => AgentWasmRuntime;
  start: () => void;
  default: () => void; // WASM initialization function
}
```

### AgentWasmRuntime Interface

Your Rust `AgentWasmRuntime` should implement these methods (already defined in your `src/lib.rs`):

```rust
#[wasm_bindgen]
impl AgentWasmRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new(jwt: String) -> AgentWasmRuntime;
    
    #[wasm_bindgen]
    pub fn start(&mut self);
    
    #[wasm_bindgen]
    pub async fn chat(&self, messages: Vec<Message>, cursor_position: usize, note: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen]
    pub fn is_running(&self) -> bool;
}
```

## File Structure After WASM Build

```
pkg/
├── src/
│   ├── wasm/
│   │   ├── aimo_note_agent.js          # Generated JS bindings
│   │   ├── aimo_note_agent_bg.wasm     # The actual WASM binary
│   │   ├── aimo_note_agent_bg.wasm.d.ts
│   │   └── package.json
│   ├── types.ts
│   ├── context/
│   ├── hooks/
│   ├── utils/
│   └── index.ts
└── lib/                                # Built SDK files
```

## Integration Steps

### 1. Update Build Process

Add WASM build to your package.json scripts:

```json
{
  "scripts": {
    "build:wasm": "wasm-pack build --target bundler --out-dir pkg/src/wasm --out-name aimo_note_agent",
    "build:sdk": "tsc && rollup -c",
    "build": "npm run build:wasm && npm run build:sdk",
    "dev": "npm run build:wasm && npm run dev:sdk",
    "dev:sdk": "tsc --watch"
  }
}
```

### 2. Update Rollup Configuration

Modify `rollup.config.js` to handle WASM files:

```javascript
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const pkg = require('./package.json');

export default [
  {
    input: 'src/index.ts',
    output: {
      file: pkg.main,
      format: 'cjs',
      sourcemap: true,
      exports: 'named',
    },
    external: ['react', 'react-dom', ...Object.keys(pkg.peerDependencies || {})],
    plugins: [
      resolve({
        browser: true,
        preferBuiltins: false,
      }),
      commonjs(),
      typescript({
        tsconfig: './tsconfig.json',
        declaration: true,
        declarationDir: 'lib',
        rootDir: 'src',
      }),
    ],
  },
  // ... ES modules build
];
```

### 3. Update Context for WASM Loading

The `AgentContext.tsx` already handles WASM loading gracefully. When you have the actual WASM module, it will be loaded automatically.

### 4. Global WASM Declaration

Add to `src/global.d.ts`:

```typescript
declare global {
  interface Window {
    WasmModule?: {
      AgentWasmRuntime: new (jwt: string) => import('./types').AgentWasmRuntime;
      start: () => void;
    };
  }
}

export {};
```

## Usage Example with Real WASM

```tsx
import React from 'react';
import { AgentProvider, useAgent, useChat } from '@aimoverse/note-agent-react';

function App() {
  return (
    <AgentProvider 
      config={{ 
        jwt: 'your-actual-jwt-token',
        autoStart: true 
      }}
      onError={(error) => console.error('Agent error:', error)}
      onStatusChange={(status) => console.log('Agent status:', status)}
    >
      <ChatInterface />
    </AgentProvider>
  );
}

function ChatInterface() {
  const { status, isReady, error } = useAgent();
  const { sendMessage, messages, isLoading } = useChat();
  
  // This will now work with your actual WASM agent
  const handleSendMessage = async () => {
    const note = NoteUtils.createEmptyNote('example');
    const action = await sendMessage('Hello, real agent!', 0, note);
    console.log('Agent responded:', action);
  };
  
  return (
    <div>
      <div>Status: {status}</div>
      <div>Ready: {isReady ? 'Yes' : 'No'}</div>
      {error && <div>Error: {error.message}</div>}
      
      <button onClick={handleSendMessage} disabled={!isReady || isLoading}>
        Send Message to Real Agent
      </button>
      
      <div>
        {messages.map((msg, index) => (
          <div key={index}>{msg.role}: {msg.content}</div>
        ))}
      </div>
    </div>
  );
}
```

## Troubleshooting

### Common Issues

1. **WASM module not found:**
   - Ensure `wasm-pack build` was run successfully
   - Check that files exist in `pkg/src/wasm/`
   - Verify the import path in `AgentContext.tsx`

2. **TypeScript errors:**
   - Make sure type definitions are generated by wasm-pack
   - Add `@types/wasm-bindgen` if needed

3. **Runtime errors:**
   - Check browser console for WASM loading errors
   - Ensure your JWT token is valid
   - Verify WASM module exports match expected interface

### Development Tips

1. **Hot Reloading:**
   ```bash
   # Terminal 1: Watch WASM changes
   npm run build:wasm
   
   # Terminal 2: Watch TypeScript changes  
   npm run dev:sdk
   ```

2. **Debugging:**
   ```tsx
   import { DebugUtils } from '@aimoverse/note-agent-react';
   
   // Use debug utilities
   DebugUtils.logAgentStatus(status, error);
   DebugUtils.logNoteStructure(note);
   ```

3. **Testing:**
   ```tsx
   // Test with mock data first
   const mockNote = NoteUtils.createEmptyNote('test');
   DebugUtils.logNoteStructure(mockNote);
   ```

## Performance Considerations

1. **Lazy Loading:** The WASM module is loaded asynchronously
2. **Memory Management:** Large notes may impact performance
3. **Bundle Size:** WASM files add to your bundle size
4. **Browser Compatibility:** Ensure WASM support in target browsers

## Next Steps

1. Build your WASM module: `npm run build:wasm`
2. Test the integration: `npm run build && npm test`
3. Deploy your application with the full SDK + WASM bundle

The SDK is designed to gracefully handle missing WASM modules during development and provide full functionality when the real WASM module is available. 