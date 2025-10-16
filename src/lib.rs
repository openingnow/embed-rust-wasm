#![allow(non_upper_case_globals)]

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlTextAreaElement, wasm_bindgen, window};

const document: fn() -> Document = || window().unwrap().document().unwrap();
const create_element: fn(&str) -> Element = |s| document().create_element(s).unwrap();

#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn run() {
    let button: HtmlElement = create_element("button").unchecked_into();
    button.set_text_content(Some("Click!"));
    let textarea: HtmlTextAreaElement = create_element("textarea").unchecked_into();
    textarea.set_value("This is textarea");
    document()
        .body()
        .unwrap()
        .append_with_node_2(&button, &textarea)
        .unwrap();
    web_sys::console::log_1(&"console output from rust!!".into());
}
