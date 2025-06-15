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
