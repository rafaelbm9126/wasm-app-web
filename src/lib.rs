mod utils;

use wasm_bindgen::prelude::*;

use serde::{ Serialize, Deserialize };

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello world!");
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Form {
    #[wasm_bindgen(skip)]
    pub name: String,
    pub age: u8,
}

#[wasm_bindgen]
impl Form {

    #[wasm_bindgen(constructor)]
    pub fn new (val1: String, val2: u8) -> Form {
        Form { name: val1, age: val2 }
    }

    #[wasm_bindgen(getter)]
    pub fn get_name (&self) -> String {
        log( &*(format!("Internal: {}", self.name )) );
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_age (&self) -> u8 {
        self.age
    }

    #[wasm_bindgen(setter)]
    pub fn set_name (&mut self, val: String) {
        self.name = val
    }

    #[wasm_bindgen(setter)]
    pub fn set_age (&mut self, val: u8) {
        self.age = val
    }
}

// log( &*(format!("new value: {}", self.name.clone() )) );
