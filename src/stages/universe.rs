use wasm_bindgen::prelude::*;

// In POC of technology universe contain two locations and one chest
// By default chest contain one item, which player can take to inventory

use crate::{actors::inventory_item::InvenotoryItem, actors::player::Player, stages::place::Place};


#[wasm_bindgen]
pub struct Universe {
    chest: Option<InvenotoryItem> // If chest empry -> none, else invenotory item
}

#[wasm_bindgen]
pub enum PlayerActions {
    GoToDefaultRoom,
    GoToAnotherRoom,
    GetItemFromChest,
    PutItemToChest,
    BoastTheItem
}

#[wasm_bindgen]
impl Universe {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { chest: Some(InvenotoryItem::new_important()) } // Put item into chest
    }

    pub fn create_player(&self) -> Player {
        return Player::new(Place::DefaultRoom);
    }

    pub fn available_actions(&self, player: &Player) -> Vec<PlayerActions> {
        let mut res = Vec::<PlayerActions>::new();
        if player.position == Place::DefaultRoom {
            if player.have_item(InvenotoryItem::important_name()) {
                res.push(PlayerActions::BoastTheItem);
            }
            res.push(PlayerActions::GoToAnotherRoom);
        }
        if player.position == Place::AnotherRoom {
            match self.chest {
                None => {
                    if player.have_item(InvenotoryItem::important_name()) {
                        res.push(PlayerActions::PutItemToChest);
                    }
                    // else - proebal
                },
                Some(_) => {
                    res.push(PlayerActions::GetItemFromChest);
                }
            }
            res.push(PlayerActions::GoToDefaultRoom);
        }
        res
    }

    // Player and world can change by actions => nut reference
    pub fn use_action(&mut self, player: &mut Player, action: PlayerActions) {
        match action {
            PlayerActions::GoToDefaultRoom  => {
                player.position = Place::DefaultRoom;
            },
            PlayerActions::GetItemFromChest => {
                let item = self.chest.take().unwrap(); // We know chest contains item. In another case - panic. `Take` place None on chest
                player.add_item_to_inventory(item);
            },
            PlayerActions::PutItemToChest => {
                let item = player.get_item_from_inventory().unwrap(); // Same case. We know: player have item
                self.chest = Some(item);
            },
            PlayerActions::GoToAnotherRoom => {
                player.position = Place::AnotherRoom;
            },
            PlayerActions::BoastTheItem => {
                // Do nothing
            }
        }
    }



}