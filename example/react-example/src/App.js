import * as React from "react";
import logo from './logo.svg';
import './App.css';
import init, { Cell, Range } from "./lib/pkg";

function App() {
  const [output, setOutput] = React.useState("");
  const [rangeOutput, setRangeOutput] = React.useState("");

  React.useEffect(() => {
    init().then(
      () => {
        try {
          // Cell operation
          let myCell11 = new Cell(0, 0, undefined);
          let myCell12 = new Cell(12, 12, undefined);

          let addr1 = myCell11.to_str_address();
          let addr2 = myCell12.to_str_address();

          let msg = `Defined 2 different cells ${addr1} and ${addr2}`;

          setOutput(msg);

          // Range operation
          let myRange1 = new Range(myCell11, myCell12);
          
          let rangeMsg = `Defined a single range with cells. ${myRange1.to_str_address()}`;

          setRangeOutput(rangeMsg);
          
        } catch (err) {
          console.error("Wasm function error:", err);
          throw err;
        }
      }
    )
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          I am using web assembly file
        </p>

        <p>
          This is the loaded cell {output}
        </p>

        <p>
          This is the loaded Range {rangeOutput}
        </p>
      </header>
    </div>
  );
}

export default App;
