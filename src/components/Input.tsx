import { ChangeEvent, useState } from "react";
import config from "../config";
import JSZip from "jszip";
import genInput from "../utils/genInput";


type InputProps = {
    seed: number,
    setSeed: React.Dispatch<React.SetStateAction<number>>
};

function Input({
    seed,
    setSeed
}: InputProps) {
    const [inputText, setInputText] = useState("");
    const [caseNum, setCaseNum] = useState(100);
    const [progress, setProgress] = useState(0);

    const handleDownload = async () => {
        try {
            setProgress(0);

            const zip = new JSZip();
            for (let i = 0; i < caseNum; i++) {
                const data = await genInput(seed);
                zip.file(`in_${String(i).padStart(4, "0")}.txt`, data);
            }
            const content = await zip.generateAsync(
                { type: "blob" },
                meta => { setProgress(meta.percent); }
            );

            const url = window.URL.createObjectURL(content);
            const a = document.createElement("a");
            a.href = url;
            a.download = `input_${seed}.zip`
            a.click();

            window.URL.revokeObjectURL(url);

            setProgress(0);
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
                            seed = Number(e.target.value);
                            setSeed(seed);
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
                    value={progress ? `${progress}% downloaded` : "Download"}
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