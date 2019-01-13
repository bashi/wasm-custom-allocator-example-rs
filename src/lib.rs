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
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

#[wasm_bindgen]
pub fn get_memory_size() -> usize {
    alloc::get_memory_size()
}

#[wasm_bindgen]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    use std::cmp::min;
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..(n + 1) {
        dp[i][0] = i
    }
    for j in 0..(m + 1) {
        dp[0][j] = j
    }

    for (i, a) in s.chars().enumerate() {
        for (j, b) in t.chars().enumerate() {
            dp[i + 1][j + 1] = if a == b {
                dp[i][j]
            } else {
                min(min(dp[i][j + 1], dp[i + 1][j]), dp[i][j]) + 1
            };
        }
    }
    dp[n][m]
}
