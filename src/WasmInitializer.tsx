import { useEffect, useState } from "react";
import { init } from "./wasm/rust-wasm";
import App from "./App";

function WasmInitializer() {
    const [isInitialized, setIsInitialized] = useState(false);

    useEffect(() => {
        const wasmInitialize = async () => {
            await init();
            setIsInitialized(true);
        };
        wasmInitialize()
            .catch(e => {
                console.error("wasm の初期化に失敗しました", e);
            });
    }, []);

    return isInitialized
        ? <App />
        : (
            <>
                <p>初期化中...</p>
            </>
        );
}

export default WasmInitializer;