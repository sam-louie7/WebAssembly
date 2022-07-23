
extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/domUtils.js")]
extern "C"{
    fn appendStringToBody(s: &str);
    fn addTwoNumbers(x: i32, y: i32) -> i32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg : &str);
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
    appendStringToBody("HelloWorld!!!");
    let r: i32= addTwoNumbers(42, 36);
    log!("The result is : {}", r);

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

