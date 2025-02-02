import { Ret } from "../../rust-wasm/pkg/tools";

type VisualizerProps = {
    result: Ret
}

function Visualizer({ result } : VisualizerProps) {
    
    return (
        <>
            <p>{`Score: ${result?.score ?? 0}`}</p>
        </>
    );
};

export default Visualizer;