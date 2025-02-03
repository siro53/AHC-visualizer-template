import React from "react";

type FileUploaderProps = {
    outputFiles: File[];
    onChangeFileSelector: React.ChangeEventHandler<HTMLSelectElement>;
    onChangeDirectoryUploader: React.ChangeEventHandler<HTMLInputElement>;
};

function FileUploader({
    outputFiles,
    onChangeFileSelector,
    onChangeDirectoryUploader
}: FileUploaderProps) {
    return (
        <> 
            <p>
                <label
                    id="directory-uploader"
                >
                    {"入力に対応する出力ファイルを選択してください: "}
                    <select 
                        id="file-selector"
                        disabled={outputFiles.length === 0}
                        onChange={onChangeFileSelector}
                    >
                        {outputFiles.map((file, index) => {
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
                    onChange={onChangeDirectoryUploader}
                    {...{ webkitdirectory: "true", directory: "true" }}
                />
            </p>
        </>
    );
};

export default FileUploader;