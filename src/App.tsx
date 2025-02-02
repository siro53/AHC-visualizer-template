import { useEffect, useState } from "react"
import FileUploader from "./components/FileUploader"
import Usage from "./components/Usage"
import Output from "./components/Output";
import Input from "./components/Input";
import Visualizer from "./components/Visualizer";
import { Ret } from "../rust-wasm/pkg/tools";

function App() {
	const [selectedFile, setSelectedFile] = useState<File | undefined>(undefined);
	const [seed, setSeed] = useState(0);
	const [result, setResult] = useState<Ret>();

	useEffect(() => {

	}, [selectedFile, seed]);

	return (
		<>
			<Usage />
			<FileUploader setSelectedFile={setSelectedFile}/>
			<Input seed={seed} setSeed={setSeed} />
			<Output />
			<Visualizer result={result} />
		</>
	)
}

export default App
