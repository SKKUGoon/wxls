# Rosetta

WebAssembly utility file for Microsoft Office (Excel) JS API. WXLS originally stands fofr `WASM XLS`, for `XLS` used for abbreviating Microsoft Excel. 

* Project name changed from WXLS to Rosetta. 

## 1. Structure

* `Cell` - Building block of Rosetta. Consists of row, column, sheet name and anchoring condition for each cells.
    ```rust
    #[wasm_bindgen]
    #[derive(Debug, Default, Clone)]
    pub struct Cell {
        /// Row start from index 0. For example, 0 => Excel Row 1, 1 => Excel Row 2,
        pub row: u32,
        /// Column start from index 0. For example, 0 => A, 1 => B, ...
        pub column: u32,
        /// Optional sheet name. If `None`, the cell is considered local.
        /// It's recommended to use the `sheet` option as Microsoft Office JS API's autofill functions may prune `None` sheet cells.
        #[wasm_bindgen(getter_with_clone)]
        // WASM package should support clone. Make Clone with (`getter_with_clone`)
        pub sheet: Option<String>,

        /// Indicates if the row is anchored. When true, A1 becomes A$1.
        pub fixed_row: bool,
        /// Indicates if the column is anchored. When true, A1 becomes $A1.
        pub fixed_column: bool,
    }
    ```

* `Range` - `Range` is consisted of two `Cell`s. If the sheet name of both `Cell`s are not matching, it will emit error.

    ```rust
    #[wasm_bindgen]
    #[derive(Debug, Default, Clone)]
    pub struct Range {
        #[wasm_bindgen(getter_with_clone)]
        pub cell_start: Cell,

        #[wasm_bindgen(getter_with_clone)]
        pub cell_end: Cell,

        pub columns: u32,
        pub rows: u32,
        pub cells: u32,
    }
    ```

* `Mathmatics` - On building process

* `WebExcelError` - Rosetta project's dedicated custom error. Mind that one needs to implement `Into<JsValue>`.
    ```rust
    impl Into<JsValue> for WebExcelError {
        // Need `Into<JsValue>`
        fn into(self) -> JsValue {
            // Convert the error enum into a string representation
            let error_message = format!("{}", self);
            JsValue::from_str(&error_message)
        }
    }

    ```

## 2. How to use it in project?

For now, the project has not gone into `npm`. You'll have to download manually. The following command will create a `./pkg` directory with `*.wasm` build file.

```console
wasm-pack build --target web
```

```console
wasm-pack test --node
```

1. Copy the entire `./pkg` file into your project's directory tree
    * For example, if you are using svelte-kit, copy the whole `pkg` directory into `./src/lib` folder. 
2. Call the `wasm` by using `import * as wasm from $lib/pkg` script.

### Example usage on React

See the example page on `example/react-example` [here]("./example/react-example")


## 3. WASM support pages

* Regarding `wasm-test`. [Testing](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html).
  * Test wasm functions uing `wasm-pack test --node`.

* Regarding JsValues for Vec and Arrays in Rust. [JsValue](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html#method.from_serde).

* How to handle not-Copyable data types. [Cloning](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter_with_clone.html).