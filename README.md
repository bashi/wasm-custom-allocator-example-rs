# Custom allocator example for WebAssembly in Rust

## Build

```sh
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --browser --no-modules target/wasm32-unknown-unknown/release/wasm_custom_allocator_test.w
asm --out-dir ./dist
```