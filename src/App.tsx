import { useState } from "react"
import FileUploader from "./components/FileUploader"
import Usage from "./components/Usage"
import Output from "./components/Output";
import Input from "./components/Input";
import { VisualizerInfo } from "./types";

function App() {
	const [seed, setSeed] = useState(0);
	const [outputFiles, setOutputFiles] = useState<File[]>([]);
	const [visualizerInfo, setVisualizerInfo] = useState<VisualizerInfo>({
		input: "",
		output: "",
		turn: 0
	});

	const convertFileToOutput = (file: File) => {
		const reader = new FileReader();
		reader.onload = e => {
			// readAsText を呼ぶので as string してもよい
			const textData = e.target?.result as string;
			setVisualizerInfo({
				...visualizerInfo,
				output: textData
			});
		}
		reader.readAsText(file);
	};

	return (
		<>
			<Usage />
			<FileUploader
				outputFiles={outputFiles}
				onChangeDirectoryUploader={e => {
					const newFiles = e.target.files ? Array.from(e.target.files) : [];

					// アップロードしたディレクトリ内にテキストファイルがなかったら何もしない
					if (newFiles.length === 0) return;

					newFiles.sort((a, b) => a.name.localeCompare(b.name));
					setOutputFiles(newFiles);
					convertFileToOutput(newFiles[0]);
				}}
				onChangeFileSelector={e => {
					const selectedIndex = e.target.selectedIndex;
					convertFileToOutput(outputFiles[selectedIndex]);
				}}
			/>
			<Input
				seed={seed}
				onChangeSeed={e => { setSeed(Number(e.target.value)); }}
			/>
			<Output />
		</>
	)
}

export default App
