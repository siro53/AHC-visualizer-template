import init, { gen } from "../../rust-wasm/pkg/tools";

function genInput(seed: number) {
    return init()
        .then(() => {
            return gen(seed);
        });
}

export default genInput;