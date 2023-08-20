# WXLS

<h3>WebAssembly for Microsoft Office(Excel) JS API. This repository directly aims to create a webpack for Excel JS API. 
`wxls` stands for `Wasm XLS` (of course `xls` is used for Mirosoft Excel's file extensions)<h3>

## How to use?

For now, the project has not gone into `npm`. You'll have to download manually. 
The following command will create a `./pkg` directory with `*.wasm` build file.

```console
wasm-pack build --target web
```

```console
wasm-pack test --node
```

Copy the entire `./pkg` file into your project's directory tree

* For example, if you are using svelte-kit, copy the whole `pkg` directory into `./src/lib` folder. 
* Call the `wasm` by using `import * as wasm from $lib/pkg` script.


## Functions

Excel JS API is consisted with following hierarchy. 
```console
workbook              // Your typical opened excel program

    └─ worksheet

        └─ Range      // Cell range. Such as (A1:C1)

            └─ Cell   // Cell address (A1)
```


### Address
Each `Cell` have String type `address` such as (A1) and `row-column address`, that's automatically converted into string address within excel. 

`WXLS` use row column address as a default, because it's much easier to manipulate around by adding integers, but generates string typed address whenever it's needed via `to_cell_address` methods implemented in the struct. 

### Cells


`wxls` aims to allow users to 
1. Create function over designated area.
2. Quickly move cursors to designated location. 


### WASM support page

* https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html

* Regarding `wasm-test`. https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html. 
  * Test wasm functions uing `wasm-pack test --node`