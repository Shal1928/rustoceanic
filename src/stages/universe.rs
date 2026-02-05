use wasm_bindgen::prelude::*;

// In POC of technology universe contain two locations and one chest
// By default chest contain one item, which player can take to inventory

use crate::{actors::{inventory_item::{InvenotoryItem, InventorySpecial}, player::{Player, PlayerActions}}, stages::{description::{Description}, place::Place}};


#[wasm_bindgen]
pub struct Universe {
    description: Option<Description>, // If chest empty -> none, else invenotory item
}

static COMPILYATOR_9000: &'static str = "Компилятор 9000";

#[wasm_bindgen]
impl Universe {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut desc: Description = Description::new("title1", "explain title1", "открылся скрытый проход", InventorySpecial::Run);
        let imp_item: InvenotoryItem = InvenotoryItem::new(&COMPILYATOR_9000, InventorySpecial::Run);
        desc.add_item(imp_item);

        Self { 
            description: Some(desc),
        }
        
    }

    pub fn create_player(&self) -> Player {
        return Player::new(Place::DefaultRoom);
    }

    pub fn get_description(&self) -> Option<Description> {
        return self.description.clone();
    }

    pub fn available_actions(&self, player: &Player) -> Vec<PlayerActions> {
        let mut res: Vec<PlayerActions> = Vec::<PlayerActions>::new();
        
        if player.position == Place::DefaultRoom {
            if player.have_item_by_name(&COMPILYATOR_9000) {
                res.push(PlayerActions::BoastTheItem);
            }

            res.push(PlayerActions::GoToAnotherRoom);
        }
        
        if player.position == Place::AnotherRoom {
            
            if player.have_item_by_name(&COMPILYATOR_9000) {
                res.push(PlayerActions::PutItemToChest);
            } else {
                res.push(PlayerActions::GetItemFromChest);
            }

            res.push(PlayerActions::GoToDefaultRoom);
        }

        if player.have_item_by_name(&COMPILYATOR_9000) {
            res.push(PlayerActions::Use);
        }

        res
    }

    // Player and world can change by actions => mut reference
    //
    pub fn use_action(&mut self, player: &mut Player, action: PlayerActions) {
        match action {
            PlayerActions::GoToDefaultRoom  => {
                player.position = Place::DefaultRoom;
            },
            PlayerActions::GetItemFromChest => {
                match self.description.as_mut() {
                    Some(desc) => {
                        player.add_item_to_inventory(&desc.items().pop().take().unwrap());
                    },
                    None => { /* empty chest */ }
                }    
            },
            PlayerActions::PutItemToChest => {
                
                match player.right_hand().take() {
                    Some(item_in_rhand) => {
                       player.get_item_from_inventory(&item_in_rhand).take().unwrap();

                        match self.description.as_mut() {
                            Some(desc) => {
                                desc.items().push(item_in_rhand);
                            },
                            None => { /* dropped */ }
                        }
                    },
                    None => { /* dropped */ }
                }
            },
            PlayerActions::GoToAnotherRoom => {
                player.position = Place::AnotherRoom;
            },
            PlayerActions::BoastTheItem => {
                // Do nothing
            },
            PlayerActions::Use => {
                let special = player.use_item(&COMPILYATOR_9000);
                
                match self.description.as_mut() {
                    Some(desc) => { desc.check_activate(special) },
                    None => { unreachable!() }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_can_do_basic() {
        let mut universe = Universe::new();
        let mut player = universe.create_player();
        universe.use_action(&mut player, PlayerActions::GoToAnotherRoom);
        universe.use_action(&mut player, PlayerActions::GetItemFromChest);
        let avalible_actions = universe.available_actions(&mut player);
        let use_action = avalible_actions.iter().find(|action| **action == PlayerActions::Use);
        assert_eq!(use_action, Some(&PlayerActions::Use));
        let universe_description = universe.get_description();
        assert_eq!(universe_description.unwrap().activate(), "Nothing happened");
        universe.use_action(&mut player, PlayerActions::Use);
        let universe_description = universe.get_description();
        assert_ne!(universe_description.unwrap().activate(), "Nothing happened");

    }
}