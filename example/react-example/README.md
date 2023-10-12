# Example code for using WASM in React App

Step 1. Copy the rust build file from `./pkg` to `./example/react-example/src/lib/pkg`

Step 2. Move into the npm root `cd ./example/react-example`

Step 3. Notice that, `./pkg` has `package.json`. Install into npm using `npm install ./lib/pkg` 

Step 4. Editting `App.js`. Import `React`. Also import builded web assembly project using `import init, { Cell, Range } from "./lib/pkg";`. 
  * `init` is used for ensuring that `WASM`is loaded into package. 
  * From now, use it inside `React.useEffect`. Initiate the web assembly file with `init()` and keep writing the functionalities using `.then(() => { ... })`.
  ```javascript
    React.useEffect(() => {
        init().then(
            () => {
                try {
                    // Cell operation
                    let myCell11 = new Cell(0, 0, undefined);
                    let myCell12 = new Cell(12, 12, undefined);

                    /* Write your rest of the code */
                    
                } catch (err) {
                    console.error("Wasm function error:", err);
                    throw err;
                }
            }
        )
    }, []);
  ```

  Step 5. `npm start` to see the rendered client. ![alt text](./public/Screenshot%202023-10-12%20at%202.54.31 PM.png)