import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import WasmInitializer from './WasmInitializer.tsx'

createRoot(document.getElementById('root')!).render(
	<StrictMode>
		<WasmInitializer />
	</StrictMode>,
)
