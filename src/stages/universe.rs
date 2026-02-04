use wasm_bindgen::prelude::*;
use web_sys::js_sys;

// In POC of technology universe contain two locations and one chest
// By default chest contain one item, which player can take to inventory

use crate::{actors::{inventory_item::{InvenotoryItem, InventorySpecial}, player::{Player, PlayerActions}}, stages::{description::{self, Description}, place::Place}};


#[wasm_bindgen]
pub struct Universe {
    description: Option<Description>, // If chest empty -> none, else invenotory item
    on_update: Option<js_sys::Function>
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
            on_update: None, 
        }
        
    }

    pub fn create_player(&self) -> Player {
        return Player::new(Place::DefaultRoom);
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

    // Player and world can change by actions => nut reference
    //
    pub fn use_action(&mut self, player: &mut Player, action: PlayerActions) {
        match action {
            PlayerActions::GoToDefaultRoom  => {
                player.position = Place::DefaultRoom;
            },
            PlayerActions::GetItemFromChest => {
                match self.description.as_ref() {
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

                        match self.description.as_ref() {
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

                if let Some(ref callback) = self.on_update {
                    let this = JsValue::null();
                    let mut activate_info = "There's nothing to apply here".to_string();

                    //тут мне надо объяснить - нихуя не понятно, но очень инетерсно
                    match self.description.as_ref() {
                        Some(desc) => {
                            activate_info = desc.check_activate(special);
                        },
                        None => { /* not described room */ }
                    }

                    let _ = callback.call1(&this, &JsValue::from(activate_info));
                }
            }
        }
    }

     #[wasm_bindgen]
    pub fn set_on_update(&mut self, callback: js_sys::Function) {
        self.on_update = Some(callback);
    }
}