import { useState } from 'react';
import { FaPlay, FaBook, FaTimes, FaCode, FaDog, FaImages } from 'react-icons/fa';
import './styles/buttons.css';

function App() {
  // State to hold code
  const [code, setCode] = useState("");
  // State to store output
  const [output, setOutput] = useState("");
  // State to toggle docs popup
  const [showDocs, setShowDocs] = useState(false);
  // State to toggle dog gallery popup
  const [showDogGallery, setShowDogGallery] = useState(false);

  /**
   * This will use the backend to run the code and return an output
   */
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
    <div className="editor-container">
      <header className="editor-header">
        <div className="title-section">
          <FaCode className="title-icon" />
          <h1>FinnLang IDE</h1>
          <FaDog className="dog-icon" />
        </div>
        <div className="toolbar">
          <button onClick={runCode} className="run-button">
            <FaPlay />
            <span>Run</span>
          </button>
          <button onClick={() => setShowDocs(true)} className="docs-button">
            <FaBook />
            <span>Documentation</span>
          </button>
          <button onClick={() => setShowDogGallery(true)} className="gallery-button">
            <FaImages />
            <span>Dog Gallery</span>
          </button>
        </div>
      </header>
      {/* For the cool ahh code editor */}
      <div className="editor-layout">
        <div className="code-section">
          <div className="code-header">
            <span className="file-name">main.finn</span>
            <div className="editor-controls">
              <div className="line-numbers">Lines: {code.split('\n').length}</div>
            </div>
          </div>
          <textarea
            value={code}
            onChange={(e) => setCode(e.target.value)}
            placeholder={`// Write your .finn code here...
let greeting = "Hello, FinnLang!";
woof(greeting);`}
            className="code-input"
            spellCheck="false"
          />
        </div>

        <div className="output-section">
          <div className="output-header">
            <span>Output</span>
          </div>
          <pre className="output-box">{output || "// Click Run to see output here..."}</pre>
        </div>
      </div>
      {/* This is for the documentation popup */}
      {showDocs && (
        <div className="modal">
          <div className="modal-content docs-modal">
            <div className="modal-header">
              <h2><FaBook className="modal-icon" /> FinnLang Documentation</h2>
              <button className="close-button" onClick={() => setShowDocs(false)}>
                <FaTimes />
              </button>
            </div>
            <div className="docs-content">
              <h3>Types</h3>
              <ul>
                <li><code>int</code> — 64-bit signed integers</li>
                <li><code>bool</code> — true / false</li>
                <li><code>string</code> — Double-quoted strings</li>
                <li><code>double</code> — Floating point numbers</li>
              </ul>

              <h3>Expressions</h3>
              <pre>{`let a = 5 + 3;
let b = "Hello, " + "world!";
let c = (a == 8);`}</pre>

              <h3>Printing</h3>
              <pre>{`woof("Hello, world!");
let a = "HI";
woof(a);`}</pre>
              <h3>If statements</h3>
              <pre>
                {`if (x < 0){
    woof("x is less than 0");
}elif (x == 5){
    woof("x is equal to 5");
}else{
    woof("x is something else");
}
                `}
              </pre>
              <h3>While Loops</h3>
              <pre>{`let x = 0;
while (x < 5) {
    woof(x);
    x = x + 1;
}`}</pre>
              <h3>Arrays</h3>
              <pre>
                {`let nums = [0, 1, 2, 3, 4, 5];
let names = ["Alice", "Bob", "Charlie"];`}</pre>
              <h3>Assignment</h3>
              <pre>{`let count = 10;
woof(count);
count = count + 1;
woof(count);`}</pre>

              <h3>Variable Declaration</h3>
              <pre>{`let x = 10;
let name: string = "Charlie";
let name2 = "Charlie";
let flag: bool = true;`}</pre>
              <h3>Example: FizzBuzz</h3>
              <pre>{`let n = 50;
let i = 0;

while (i < n) {
  if ((i % 5 == 0) && (i % 3 == 0)) {
    woof("FizzBuzz");
  }
  elif ((i % 5 == 0)) {
    woof("Fizz");
  }
  elif ((i % 3 == 0)) {
    woof("Buzz");
  }
  else {
    woof(i);
  }

  i = i + 1;
}`}</pre>
            </div>
          </div>
        </div>
      )}

      {showDogGallery && (
        <div className="modal">
          <div className="modal-content dog-gallery-modal">
            <div className="modal-header">
              <h2><FaDog className="modal-icon" /> Dog Gallery</h2>
              <button className="close-button" onClick={() => setShowDogGallery(false)}>
                <FaTimes />
              </button>
            </div>
            {/* Most important feature right here! */}
            <div className="gallery-content">
              <div className="gallery-grid">
                <img src="../photos/finn1.jpeg" alt="Finn Eating a stick" className="gallery-photo" />
                <img src="../photos/finn2.jpeg" alt="Finn Eating a toy" className="gallery-photo" />
                <img src="../photos/finn3.jpeg" alt="Finn Eating a toy" className="gallery-photo" />
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
