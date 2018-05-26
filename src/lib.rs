#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

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
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add_paragraph(text: &str) {
    let p = document.createElement("p");
    p.set_inner_html(text);
    document.body().append_child(p);
}
