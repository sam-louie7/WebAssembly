#[macro_use]
extern crate cfg_if;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.

cfg_if! {

    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world bitch!"));

    Ok(())
}



#[wasm_bindgen]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[wasm_bindgen]
pub struct Image {
    pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Image {
    pub fn new() -> Image {
        let color1 = Color{
            red: 255,
            green: 0,
            blue: 0,
        };
        let color2 = Color{
            red: 60,
            green: 70,
            blue: 90,
        };
        let pixels = vec![color1, color2];
        Image { 
            pixels: pixels 
        }
    }

    pub fn pixels_ptr(&self) -> *const Color {
        self.pixels.as_ptr()
    }
}

#[wasm_bindgen]
extern "C" {
    type Element;
    type HTMLDocument;
    static document : HTMLDocument;

    // crate_element is a method for HTMLDocument instance as defined in the proparty
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tag_name: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html : &str);

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
}

macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub fn run() {
    let now = js_sys::Date::now();
    let now_date = js_sys::Date::new(&JsValue::from_f64(now));
    let val = document.createElement("p");
    val.set_inner_html(&format!(
        "Hello From Rust it's {}:{}",
        now_date.get_hours(),
        now_date.get_minutes()
    ));
    document.body().append_child(val);
}