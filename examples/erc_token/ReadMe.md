## Calculator contract

A simple erc 20 token contractor to do basic deployment with memory storage.


## How to compile

```
rustup target add wasm32-unknown-unknown
cargo +nightly build --target wasm32-unknown-unknown --release -Z unstable-options --out-dir ./wasm
```


## How to test

```
cargo +nightly test -Z unstable-options
```

## WASM optimization

Download and install the latest version of [binaryen](https://github.com/WebAssembly/binaryen) first.

```
wasm-opt -Os -o wasm/calculator.wasm wasm/calculator.wasm
```
