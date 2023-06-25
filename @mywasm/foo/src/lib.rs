mod utils;

use wasm_bindgen::prelude::*;
use num::bigint::BigUint;
use num::traits::{One, Zero};
use std::mem::replace;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, foo!");
}

#[wasm_bindgen]
pub fn fibonacci(num: usize) -> String {
  let mut f0: BigUint = Zero::zero();
  let mut f1: BigUint = One::one();
  for _ in 0..num {
    // f0 所有权转移给f2
    let mut f2: BigUint = f0 + &f1;
    // f1原本的所有权转移给f0, 然后f2当前的给f1
    f0 = replace(&mut f1, f2);
  }
  f0.to_string()
}
