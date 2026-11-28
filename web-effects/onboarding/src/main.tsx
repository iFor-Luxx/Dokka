import '@fontsource/pacifico/400.css';
import { StrictMode, useState } from 'react';
import { createRoot } from 'react-dom/client';

import { Hello } from './hola/Hello';
import './styles.css';

function App() {
  // Cambiar la clave vuelve a empezar la secuencia desde el principio.
  const [run, setRun] = useState(0);
  return (
    <>
      <Hello key={run} />
      {import.meta.env.DEV && (
        <button type="button" className="replay" onClick={() => setRun(run + 1)}>
          Repetir
        </button>
      )}
    </>
  );
}

const root = document.getElementById('root');
if (root) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
