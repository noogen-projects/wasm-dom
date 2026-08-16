use std::iter;

pub use js_sys::{self, Reflect};
use wasm_bindgen::prelude::Closure;
pub use wasm_bindgen::{self, JsCast, JsValue, UnwrapThrowExt};
pub use web_sys::{Document, Location, Window};
use web_sys::{Element, HtmlElement, NodeList};

pub use crate::error::{Error, Result};
use crate::existing::access::CastToElement;

pub mod animation;
pub mod correct;
pub mod error;
pub mod event;
pub mod existing;

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::WindowNotFound)
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or(Error::DocumentNotFound)
}

pub fn document_element() -> Result<Element> {
    document()?.document_element().ok_or(Error::DocumentElementNotFound)
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

pub fn select_element_from(root: &Element, selectors: &str) -> Result<Element> {
    root.query_selector(selectors)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))?
        .ok_or_else(|| Error::ElementNotFound(selectors.into()))
}

pub fn select_element(selectors: &str) -> Result<Element> {
    document()?
        .query_selector(selectors)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))?
        .ok_or_else(|| Error::ElementNotFound(selectors.into()))
}

pub fn select_all_elements_from(root: &Element, selectors: &str) -> Result<impl Iterator<Item = Element>> {
    root.query_selector_all(selectors)
        .map(elements)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))
}

pub fn select_all_elements(selectors: &str) -> Result<impl Iterator<Item = Element>> {
    document()?
        .query_selector_all(selectors)
        .map(elements)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))
}

pub fn select_element_cast<T: JsCast>(selectors: &str) -> Result<T> {
    select_element(selectors)?
        .dyn_into::<T>()
        .map_err(Error::ElementNotCast)
}

pub fn elements(list: NodeList) -> impl Iterator<Item = Element> {
    let mut index = 0;
    iter::from_fn(move || {
        if index < list.length() {
            let node = list.get(index);
            index += 1;
            Some(node)
        } else {
            None
        }
    })
    .filter_map(|node| node.and_then(CastToElement::maybe_into_element))
}

/// `setTimeout(func, delay_ms)` for a one-shot callback.
pub fn set_timeout(func: impl FnOnce() + 'static, delay_ms: i32) -> Result<i32> {
    let callback = Closure::once_into_js(func).unchecked_into();
    window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(&callback, delay_ms)
        .map_err(|_| Error::FailedToSetTimeout)
}
