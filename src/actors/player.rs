use std::{collections::HashMap, ops::{SubAssign}};

use wasm_bindgen::prelude::*;

use crate::{actors::inventory_item::{InvenotoryItem, InventorySpecial}, stages::place::Place};

#[wasm_bindgen]
pub enum PlayerActions {
    GoToDefaultRoom,
    GoToAnotherRoom,
    GetItemFromChest,
    PutItemToChest,
    BoastTheItem,
    Use
}

#[wasm_bindgen]
pub struct Player {
    name: String,
    inventory: HashMap<InvenotoryItem, u32>,
    pub position: Place,
    right_hand : Option<InvenotoryItem>
}

// Can't use from wasm
impl Player {
    pub fn new(_position: Place) -> Self {
        Self {
            name: "Player".to_string(),
            inventory: HashMap::new(),
            position: _position,
            right_hand: None
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

    pub fn have_item_by_name(&self, item_name: &str) -> bool {
        self.inventory.keys().any(|item| item.name() == item_name)
    }
}

#[wasm_bindgen]
impl Player {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn right_hand(&self) -> Option<InvenotoryItem> {
        self.right_hand.clone()
    }

    pub fn use_item(&mut self, item_name: &str) -> InventorySpecial {
         
        //linq style :-) ha-ha-ha
        self.right_hand = self.inventory
        .keys()
        .find(|item| item.name() == item_name).cloned();

        match &self.right_hand {
            Some(item) => item.special(), 
            None => InventorySpecial::Impossible,
        }
    }
}