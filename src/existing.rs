pub mod access;

use wasm_bindgen::{JsCast, UnwrapThrowExt, throw_str};
use web_sys::{Document, Element, HtmlElement, Location, Window};

pub use self::access::{Cast, JsObjectAccess};
use crate::elements;

pub fn window() -> Window {
    super::window().expect_throw("Should have a window in this context")
}

pub fn document() -> Document {
    window().document().expect_throw("Window should have a document")
}

pub fn document_element() -> Element {
    document()
        .document_element()
        .expect_throw("Document should have a root element")
}

pub fn body() -> HtmlElement {
    document().body().expect_throw("Document should have a body")
}

pub fn location() -> Location {
    document().location().expect_throw("Document should have a location")
}

pub fn get_element_by_id<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| throw_str(&format!("Document should have an element with id = `{id}`")))
        .dyn_into::<T>()
        .unwrap_or_else(|_| throw_str(&format!("Element with id = `{id}` should cast to target type")))
}

pub fn select_element_from(root: &Element, selectors: &str) -> Element {
    root.query_selector(selectors)
        .unwrap_or_else(|value| throw_str(&format!("Specified selectors = `{selectors}` is invalid: {value:?}")))
        .unwrap_or_else(|| {
            throw_str(&format!(
                "Document should have an element accessible by selectors = `{selectors}`"
            ))
        })
}

pub fn select_element(selectors: &str) -> Element {
    document()
        .query_selector(selectors)
        .unwrap_or_else(|value| throw_str(&format!("Specified selectors = `{selectors}` is invalid: {value:?}")))
        .unwrap_or_else(|| {
            throw_str(&format!(
                "Document should have an element accessible by selectors = `{selectors}`"
            ))
        })
}

pub fn select_all_elements_from(root: &Element, selectors: &str) -> impl Iterator<Item = Element> {
    root.query_selector_all(selectors)
        .map(elements)
        .unwrap_or_else(|value| throw_str(&format!("Specified selectors = `{selectors}` is invalid: {value:?}")))
}

pub fn select_all_elements(selectors: &str) -> impl Iterator<Item = Element> {
    document()
        .query_selector_all(selectors)
        .map(elements)
        .unwrap_or_else(|value| throw_str(&format!("Specified selectors = `{selectors}` is invalid: {value:?}")))
}

pub fn select_element_cast<T: JsCast>(selectors: &str) -> T {
    select_element(selectors).dyn_into::<T>().unwrap_or_else(|element| {
        throw_str(&format!(
            "Element to select by `{selectors}` should cast to target type: {element:?}"
        ))
    })
}
