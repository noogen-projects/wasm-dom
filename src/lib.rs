pub mod error;
pub mod existing;

pub use wasm_bindgen::{UnwrapThrowExt, JsCast, JsValue};
pub use web_sys::{Document, Window};
pub use js_sys::Reflect;
pub use crate::error::*;

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::WindowNotFound)
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or(Error::DocumentNotFound)
}

pub fn get_element_by_id<T: JsCast>(id: &str) -> Result<T> {
    let element = document()?
        .get_element_by_id(id)
        .ok_or_else(|| Error::ElementNotFound(id.into()))?;
    element.dyn_into::<T>().map_err(|_| Error::IsNotAnElement)
}

pub fn select_element<T: JsCast>(selectors: &str) -> Result<T> {
    document()?
        .query_selector(selectors)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))?
        .ok_or_else(|| Error::ElementNotFound(selectors.into()))?
        .dyn_into::<T>()
        .map_err(|element| Error::ElementNotCast(element))
}
