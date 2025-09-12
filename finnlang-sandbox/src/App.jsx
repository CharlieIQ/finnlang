import { useState } from 'react';
import { FaPlay, FaBook, FaTimes, FaCode, FaDog, FaImages } from 'react-icons/fa';
import './styles/buttons.css';

/**
 * This is the main app component for the FinnLang IDE. Components include:
 * - Code editor (just a simple textarea)
 * - Output display (preformatted text)
 * - Run button to execute code
 * - Documentation popup
 * - A gallery of dog photos because why not
 * 
 * I personally chose to keep everything in one file for simplicity since the frontend is so small. 
 * 
 * @returns The main app component
 */
function App() {
  // State to hold code
  const [code, setCode] = useState("");
  // State to store output
  const [output, setOutput] = useState("");
  // State to track if last execution had an error
  const [hasError, setHasError] = useState(false);
  // State to toggle docs popup
  const [showDocs, setShowDocs] = useState(false);
  // State to toggle dog gallery popup
  const [showDogGallery, setShowDogGallery] = useState(false);

  /**
   * This will use the backend to run the code and return an output
   * or error message. It sends the code to the backend via POST request
   * and updates the output state based on the response.
   */
  const runCode = async () => {
    try {
      // Send code to backend
      const res = await fetch('http://localhost:3000/run', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code }),
      });
      const data = await res.json();

      if (data.success) {
        setOutput(data.output);
        setHasError(false);
      } else {
        // Display error in a user-friendly way
        setOutput(`❌ ${data.error}`);
        setHasError(true);
      }
    } catch (error) {
      setOutput(`❌ Network Error: Failed to connect to server\n${error.message}`);
      setHasError(true);
    }
  };

  return (
    // Main container with the editor and output
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
          <pre className={`output-box ${hasError ? 'error' : ''}`}>{output || "// Click Run to see output here..."}</pre>
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

              <h3>For Loops</h3>
              <pre>{`// Basic for loop
for (let i = 0; i < 5; i = i + 1) {
    woof(i);
}

// For loop with string concatenation
for (let j = 1; j <= 3; j = j + 1) {
    woof("Count: " + j);
}`}</pre>

              <h3>Functions</h3>
              <pre>{`// Function without parameters
funct sayHello() {
    woof("Hello, world!");
}

// Function with parameters
funct greet(name: string, age: int) {
    woof("Hello " + name + ", you are " + age + " years old!");
}

// Function with return type
funct add(a: int, b: int): int {
    let result = a + b;
    woof("Result: " + result);
}

// Call functions
sayHello();
greet("Alice", 25);
add(5, 3);`}</pre>

              <h3>Comments</h3>
              <pre>{`// This is a single-line comment

/* 
   This is a multi-line comment
   that can span multiple lines
*/

let x = 5; // End-of-line comment

/* Nested comments work too:
   /* inner comment */
   Still in outer comment
*/`}</pre>

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
            <footer className="editor-footer">
              <p>Made with more coffee, CSCI2100, and love by Charlie McLaughlin</p>
              <br></br>
              <p>Also try Piggle 🐽</p>
            </footer>
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
