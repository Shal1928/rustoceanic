use wasm_bindgen::prelude::*;

use crate::{inventory_item::InvenotoryItem, place::Place};

#[wasm_bindgen]
pub struct Player {
    name: String,
    inventory: Vec<InvenotoryItem>,
    pub position: Place
}

// Can't use from wasm
impl Player {
    pub fn new(position: Place) -> Self {
        Self {
            name: "Player".to_string(),
            inventory: Vec::new(),
            position
        }
    }

    pub fn add_item_to_inventory(&mut self, item: InvenotoryItem) {
        self.inventory.push(item);
    }

    pub fn get_item_from_inventory(&mut self) -> Option<InvenotoryItem> {
        return self.inventory.pop();
    }

    pub fn have_item(&self, item_name: &str) -> bool {
        for item in &self.inventory {
            if item.name().eq(item_name) {
                return true;
            }
        }
        false
    }
}

#[wasm_bindgen]
impl Player {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}