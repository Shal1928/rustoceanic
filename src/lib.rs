use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::set_panic_hook;

mod utils;
mod actors;
mod stages;

#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();
}