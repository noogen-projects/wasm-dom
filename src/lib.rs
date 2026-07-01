pub use js_sys::{self, Reflect};
pub use wasm_bindgen::{self, JsCast, JsValue, UnwrapThrowExt};
pub use web_sys::{Document, Location, Window};
use web_sys::{Element, HtmlElement};

pub use crate::error::{Error, Result};

pub mod error;
pub mod existing;

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::WindowNotFound)
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or(Error::DocumentNotFound)
}

pub fn body() -> Result<HtmlElement> {
    document()?.body().ok_or(Error::BodyNotFound)
}

pub fn location() -> Result<Location> {
    document()?.location().ok_or(Error::LocationNotFound)
}

pub fn get_element_by_id<T: JsCast>(id: &str) -> Result<T> {
    let element = document()?
        .get_element_by_id(id)
        .ok_or_else(|| Error::ElementNotFound(id.into()))?;
    element.dyn_into::<T>().map_err(|_| Error::IsNotAnElement)
}

pub fn select_element(selectors: &str) -> Result<Element> {
    document()?
        .query_selector(selectors)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))?
        .ok_or_else(|| Error::ElementNotFound(selectors.into()))
}

pub fn select_element_as<T: JsCast>(selectors: &str) -> Result<T> {
    select_element(selectors)?
        .dyn_into::<T>()
        .map_err(Error::ElementNotCast)
}
