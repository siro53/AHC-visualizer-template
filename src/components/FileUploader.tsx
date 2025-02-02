import { ChangeEvent, useState } from "react";

type FileUploaderProps = {
    setSelectedFile: React.Dispatch<React.SetStateAction<File | undefined>>
};

function FileUploader({ setSelectedFile } : FileUploaderProps) {
    const [files, setFiles] = useState<File[]>([]);

    const handleFileSelecter = (e: ChangeEvent<HTMLSelectElement>) => {
        const selectedElement = e.target;
        setSelectedFile(files[selectedElement.selectedIndex]);
    };

    const handleDirectoryUploader = (e: ChangeEvent<HTMLInputElement>) => {
        const newFiles = e.target.files ? Array.from(e.target.files) : [];
        newFiles.sort((a, b) => a.name.localeCompare(b.name));
        setFiles(newFiles);
        setSelectedFile(newFiles.length === 0 ? undefined : newFiles[0]);
    };

    return (
        <> 
            <p>
                <label
                    id="directory-uploader"
                >
                    {"入力に対応する出力ファイルを選択してください: "}
                    <select 
                        id="file-selector"
                        disabled={files.length === 0}
                        onChange={handleFileSelecter}
                    >
                        {files.map((file, index) => {
                            return (
                                <option key={`file-selector-options-${index}`}>
                                    {file.name}
                                </option>
                            );
                        })}
                    </select>
                    {" "}
                </label>
                <input 
                    type="file"
                    id="directory-uploader"
                    onChange={handleDirectoryUploader}
                    {...{ webkitdirectory: "true", directory: "true" }}
                />
            </p>
        </>
    );
};

export default FileUploader;