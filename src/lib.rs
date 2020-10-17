mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
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
pub struct Foo {
    pub internal: i32,
}

#[wasm_bindgen]
impl Foo {

    #[wasm_bindgen(constructor)]
    pub fn new (val: i32) -> Foo {
        Foo { internal: val }
    }

    pub fn get (&self) -> i32 {
        self.internal
    }

    pub fn set (&mut self, val: i32) {
        log( &*(format!("new value: {}", val)) );
        self.internal = val;
    }
}
