use wasm_bindgen::prelude::*;

// Points in world map
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)] // It can be copy for usage in functions args.
// And equals for compare 
pub enum Place {
    DefaultRoom,
    AnotherRoom
}