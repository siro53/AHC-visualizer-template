import { useState } from "react"
import FileUploader from "./components/FileUploader"
import Usage from "./components/Usage"
import Output from "./components/Output";
import Input from "./components/Input";

function App() {
	const [_selectedFile, setSelectedFile] = useState<File | undefined>(undefined);

	return (
		<>
			<Usage />
			<FileUploader setSelectedFile={setSelectedFile}/>
			<Input />
			<Output />
		</>
	)
}

export default App
