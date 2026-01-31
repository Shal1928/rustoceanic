use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Hash, PartialEq, Eq, Clone)] // For use in hash map
pub struct InvenotoryItem {
    name: String
}

#[wasm_bindgen]
impl InvenotoryItem {
    pub fn name(&self) -> String {
        return self.name.clone();
    }
}

static IMPORTANT_ITEM_NAME: &'static str = "Very important item";

impl InvenotoryItem {
    pub fn important() -> Self {
        Self { name: IMPORTANT_ITEM_NAME.to_string() }
    }

    pub fn important_name() -> &'static str {
        return  IMPORTANT_ITEM_NAME;
    }
}