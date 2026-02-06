use wasm_bindgen::prelude::*;

use crate::actors::inventory_item::{InvenotoryItem, InventorySpecial};

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone)]
pub struct Description {
    title: String,
    explain: String,
    items: Vec<InvenotoryItem>,
    activate: String,
    activate_trigger: InventorySpecial,
    is_activated: bool
}

#[wasm_bindgen]
impl Description {

    #[wasm_bindgen(constructor)]
    pub fn new(title: &str, explain: &str, activate: &str, activate_trigger: InventorySpecial) -> Self {
        Self {
            title: title.to_string(),
            explain: explain.to_string(),
            items: Vec::new(),
            activate: activate.to_string(),
            activate_trigger,
            is_activated: false
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

    #[wasm_bindgen(getter)]
    pub fn activate(&self) -> String {
        if  self.is_activated {
            self.activate.clone()
        }
        else {
            "Nothing happened".to_string()
        }
    }

    pub fn check_activate(&mut self, activate_trigger: InventorySpecial) {
        self.is_activated = self.activate_trigger == activate_trigger;
    }
    
    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Vec<InvenotoryItem> {
        self.items.clone()
    }
}