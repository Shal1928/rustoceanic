use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum InventorySpecial {
    RestoreMemory = 1,
    FixThis = 2,
    Build = 3,
    Run = 4,
    Refactor = 5
}

#[wasm_bindgen]
#[derive(Hash, PartialEq, Eq, Clone)] // For use in hash map
pub struct InvenotoryItem {
    name: String,
    special: InventorySpecial
}

#[wasm_bindgen]
impl InvenotoryItem {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, special: InventorySpecial) -> Self {
        Self { 
            name: name.to_string(),
            special: special
        }
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

     pub fn special(&self) -> InventorySpecial {
        return self.special;
    }
}

static IMPORTANT_ITEM_NAME: &'static str = "Very important item";

impl InvenotoryItem {
    pub fn important() -> Self {
        Self { 
            name: IMPORTANT_ITEM_NAME.to_string(),
            special: InventorySpecial::Build
        }
    }

    pub fn important_name() -> &'static str {
        return  IMPORTANT_ITEM_NAME;
    }

    pub fn important_special() -> &'static InventorySpecial {
        return &InventorySpecial::Build;
    }
}