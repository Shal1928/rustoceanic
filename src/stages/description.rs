use wasm_bindgen::prelude::*;

use crate::actors::inventory_item::{InvenotoryItem, InventorySpecial};

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone)]
pub struct Description {
    title: String,
    explain: String,
    items: Vec<InvenotoryItem>,
    activate: String,
    activate_trigger: InventorySpecial
}

#[wasm_bindgen]
impl Description {

    #[wasm_bindgen(constructor)]
    pub fn new(title: &str, explain: &str, activate: &str, _activate_trigger: InventorySpecial) -> Self {
        Self {
            title: title.to_string(),
            explain: explain.to_string(),
            items: Vec::new(),
            activate: activate.to_string(),
            activate_trigger: _activate_trigger
        }
    }
    
    #[wasm_bindgen]
    pub fn add_item(&mut self, item: InvenotoryItem) {
        self.items.push(item);
    }
    
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn explain(&self) -> String {
        self.explain.clone()
    }

    pub fn check_activate(&self, _activate_trigger: InventorySpecial) -> String {
        if self.activate_trigger == _activate_trigger  {
            self.activate.clone()
        } else {
            "Nothing happened".to_string()
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Vec<InvenotoryItem> {
        self.items.clone()
    }
}