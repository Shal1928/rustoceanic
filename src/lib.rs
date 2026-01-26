mod utils;

use wasm_bindgen::prelude::*;

use web_sys;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn classic_alert() {
    unsafe {
     alert("Hello!");   
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SomeEnum {
    Var1 = 0,
    Var2 = 1,
}

#[wasm_bindgen]
pub struct SomeStructure {
    inner_vec: Vec<SomeEnum>,
}

#[wasm_bindgen]
impl SomeStructure {
    pub fn get_element(&self, i: u32) -> SomeEnum {
        self.inner_vec[i as usize]
    }

    #[wasm_bindgen(constructor)]
    pub fn new(count: u32) -> Self {
        Self {
            inner_vec: (0..count).map(|i| {
                if i % 3 == 0 {
                    SomeEnum::Var1
                } else {
                    SomeEnum::Var2
                }
            }).collect()
        }
    }

    pub fn elements_ptr(&self) -> *const SomeEnum {
        self.inner_vec.as_ptr()
    }
}

#[wasm_bindgen]
pub fn string_return_function(part: &str) -> String {
    format!("The part is {}", part)
}

#[wasm_bindgen]
pub fn error_throw_function() {
    web_sys::wasm_bindgen::throw_str("Throwing Error")
}