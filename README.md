# WasiThreadsShedYieldBug

Temporary repo to help reproduce a wasi threads bug, which only reproduces in debug mode

## Prerequisites
* Install wasmtime and dependencies https://wasmtime.dev/
* Compile the example
```
cargo build --target wasm32-wasi-preview1-threads && cargo build --release --target wasm32-wasi-preview1-threads
```

### Expected output
Running the release target shows the expected output
```
wasmtime --wasm threads --wasi threads target/wasm32-wasi-preview1-threads/release/wasi_channel.wasm
```

Output 

```
receiving
        sending [1, 2, 3]
        sent
received [1, 2, 3]
```


### Reproducing

Running the debug target reproduces the issue

```
wasmtime --wasm threads --wasi threads target/wasm32-wasi-preview1-threads/debug/wasi_channel.wasm
```

Output

```
receiving
        sending [1, 2, 3]
```