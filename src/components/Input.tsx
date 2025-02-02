import { ChangeEvent, useState } from "react";
import config from "../config";
import JSZip from "jszip";

function Input() {
    const [inputText, setInputText] = useState("");
    const [seed, setSeed] = useState(0);
    const [caseNum, setCaseNum] = useState(100);

    const handleDownload = async () => {
        try {
            const zip = new JSZip();
            zip.file('in.txt', inputText);
            const content = await zip.generateAsync({ type: "blob" });

            const url = window.URL.createObjectURL(content);
            const a = document.createElement("a");
            a.href = url;
            a.download = "in.zip"
            a.click();

            window.URL.revokeObjectURL(url);
        } catch (e) {
            console.error("zip生成に失敗しました", e);
        }
    };

    return (
        <>
            <p>
                <label>
                    {"Seed: "}
                    <input
                        type="number"
                        id="seed"
                        value={seed}
                        onChange={(e: ChangeEvent<HTMLInputElement>) => { 
                            setSeed(Number(e.target.value)); 
                        }}
                        min={config.input.seed.min}
                        max={config.input.seed.max}
                    />
                </label>
                {" "}
                <label>
                    {"#cases: "}
                    <input
                        type="number"
                        id="case"
                        value={caseNum}
                        onChange={(e: ChangeEvent<HTMLInputElement>) => {
                            setCaseNum(Number(e.target.value)); 
                        }}
                        min={config.input.cases.min}
                        max={config.input.cases.max}
                    />
                </label>
                {" "}
                <input
                    type="button"
                    id="download"
                    value="Download"
                    onClick={handleDownload}
                />
            </p>
            <p>
                {"Input: "}
                <br />
                <textarea 
                    id="input"
                    rows={config.input.rows}
                    style={config.input.textAreaStyle}
                    value={inputText}
                    onChange={(e: ChangeEvent<HTMLTextAreaElement>) => {
                        setInputText(e.target.value);
                    }}
                />
            </p>
        </>
    )
};

export default Input;