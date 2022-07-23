# Environment setup:

## 1. Add wasm32-unknown-unknown target to rustup compiler by below command.

> rustup target add wasm32-unknown-unknown

## 2. Installing wasm-bindgen
### Install rustup nightly, and webassembly bindgen command line tool 
### here we are specifically calling it from the nightly branch of rust.

> rustup toolchain install nightly
> cargo +nightly install wasm-bindgen-cli

## 3. Create a new Rust WebAssembly Project:

> cargo +nightly new  project-name --lib

## 4. Set depemdencies and lib:

[dependencies]
wasm-bindgen = "0.2.80"

[lib]
crate-type = ["cdylib"]


# Build the WebAssembly project using wasm-pack:

> wasm-pack build

## this will do the folllowing:
1. check rustc version
2. check crate config
3. add target wasm32-unknown-unknown
4. compile to .wasm 
5. create pkg/ dir.
6. writing to package.json
7. run wasm-bingen installed in dependecies.


# Run project
### 1. 
> yarn install 
### 2. 
> yarn serve
### 3. look at localhost:8080

# Publish our library:

### use wasm-pack login before publishig to login to npm (node package manager).

> wasm-pack publish

### to inatall the package in other project :

> npm install --save my-wasm-lib
> npm start

### in index.js of new wasm/js app import my-wasm-lib 

import("my-wasm-lib").then(module => {
    module.call_function(parameters);
})




