import { useState } from 'react';

function App() {
  const [code, setCode] = useState('');
  const [output, setOutput] = useState('');

  const runCode = async () => {
    const res = await fetch('http://localhost:3000/run', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code }),
    });
    const data = await res.json();
    setOutput(data.output);
  };

  return (
    <div>
      <h1>FinnLang Compiler</h1>
      <textarea
        value={code}
        onChange={(e) => setCode(e.target.value)}
        placeholder="Write your .finn code here"
        rows={10}
        cols={50}
      />
      <br />
      <button onClick={runCode}>Run</button>
      <pre>{output}</pre>
    </div>
  );
}

export default App;
