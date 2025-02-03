import { useState } from "react"
import FileUploader from "./components/FileUploader"
import Usage from "./components/Usage"
import Output from "./components/Output";
import Input from "./components/Input";

function App() {
	const [_selectedFile, setSelectedFile] = useState<File | undefined>(undefined);
	const [seed, setSeed] = useState(0);

	return (
		<>
			<Usage />
			<FileUploader setSelectedFile={setSelectedFile}/>
			<Input
				seed={seed}
				onChangeSeed={e => { setSeed(Number(e.target.value)); }}
			/>
			<Output />
		</>
	)
}

export default App
