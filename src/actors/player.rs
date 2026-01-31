use std::{collections::HashMap, ops::{SubAssign}};

use wasm_bindgen::prelude::*;

use crate::{actors::inventory_item::InvenotoryItem, stages::place::Place};

#[wasm_bindgen]
pub struct Player {
    name: String,
    inventory: HashMap<InvenotoryItem, u32>,
    pub position: Place
}

// Can't use from wasm
impl Player {
    pub fn new(position: Place) -> Self {
        Self {
            name: "Player".to_string(),
            inventory: HashMap::new(),
            position
        }
    }

    pub fn add_item_to_inventory(&mut self, item: &InvenotoryItem) {
        match self.inventory.get(item) {
            None => {
                self.inventory.insert(item.clone(), 1);
            },
            Some(count) => {
                self.inventory.insert(item.clone(), count + 1);
            }
        }
        return;
    }

    pub fn get_item_from_inventory(&mut self, item: &InvenotoryItem) -> Option<InvenotoryItem> {
        let value = self.inventory.get_mut(item);
        match value {
            None | Some(0) => None,
            Some(value) => {
                value.sub_assign(1); //Romove one item
                if *value == 0 {
                    self.inventory.remove(item);
                }
                Some(item.clone())
            }
        }
    }

    pub fn have_item(&self, item: &InvenotoryItem) -> bool {
        self.inventory.contains_key(item)
    }
}

#[wasm_bindgen]
impl Player {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}