#![cfg_attr(target_arch = "wasm32", feature(stdsimd))]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod alloc;

#[global_allocator]
static A: alloc::CustomAlloc = alloc::CustomAlloc::INIT;

#[wasm_bindgen]
pub fn add_values(arr: &[i32]) -> i32 {
    arr.into_iter().fold(0, |acc, v| acc + v)
}

#[wasm_bindgen]
pub fn get_memory_size() -> usize {
    alloc::get_memory_size()
}
