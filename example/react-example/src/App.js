import * as React from "react";
import logo from './logo.svg';
import './App.css';
import init, { Cell, Range } from "./lib/pkg";

function App() {
  const [output, setOutput] = React.useState("");
  const [outputSheet, setOutputSheet] = React.useState("");
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

          let msg1 = `Defined 2 different cells ${addr1} and ${addr2}`;

          setOutput(msg1);

          // Cell with sheet operation 
          let myCell21 = new Cell(1, 1, "Sheet1");
          let myCell22 = new Cell(100, 100, "Sheet1");

          let msg2 = `Defined 2 different cells with sheet: ${myCell21.to_str_address()} and ${myCell22.to_str_address()}`;
          setOutputSheet(msg2);

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
          This is the loaded cell: {output}
        </p>

        <p>
          This is the loaded cell with sheet: {outputSheet}
        </p>

        <p>
          This is the loaded Range: {rangeOutput}
        </p>
      </header>
    </div>
  );
}

export default App;
