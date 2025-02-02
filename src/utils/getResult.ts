import init, { vis } from "../../rust-wasm/pkg/tools";

async function getResult({
    input,
    output,
    turn
} : {
    input: string,
    output: string,
    turn: number
}) {
    return init().then(() => { return vis(input, output, turn); });
}

export default getResult;