#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./../js/pixi.min.js")]
extern {
    pub type Container;

    #[wasm_bindgen(method, getter)]
    fn x(this: &Container) -> i32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Container) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_x(this: &Container, x: i32);
    #[wasm_bindgen(method, setter)]
    fn set_y(this: &Container, x: i32);
}

#[wasm_bindgen]
extern {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
}

#[wasm_bindgen]
extern {
    /// Print to developer console
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn relocate(cont: &Container, x: i32, y: i32) {
    log(&format!("{}, {}", x, y));
    cont.set_x(x);
    cont.set_y(y);
}

#[wasm_bindgen]
pub fn print(msg: &str) {
    log(msg);
}

#[wasm_bindgen]
pub fn add_paragraph(text: &str) {
    let p = document.createElement("p");
    p.set_inner_html(text);
    document.body().append_child(p);
}
