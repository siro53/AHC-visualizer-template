import config from "../config";

function Output() {
    return (
        <>
            <p>
                {"Output: "}
                <br />
                <textarea 
                    id="output" 
                    rows={config.output.rows} 
                    style={config.output.textAreaStyle}
                />
            </p>
        </>
    );
}

export default Output;