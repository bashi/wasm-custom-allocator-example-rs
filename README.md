# Rust Custom allocator example for WebAssembly

A naive first-fit memory allocator for wasm32 target in Rust.

## Requirements
* Rust 1.33+
* wasm-bindgen CLI

Run following command to install wasm-bindgen CLI:
```sh
$ cargo install wasm-bindgen-cli
```

## Building and Testing

```sh
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --browser --no-modules target/wasm32-unknown-unknown/release/wasm_custom_allocator_example.wasm --out-dir public/dist
```

Then run an http-server with document root = `public/`. Navigate to the URL the server listening on.
