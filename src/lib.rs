pub use js_sys::{self, Reflect};
use wasm_bindgen::prelude::Closure;
pub use wasm_bindgen::{self, JsCast, JsValue, UnwrapThrowExt};
pub use web_sys::{Document, Location, Window};
use web_sys::{Element, HtmlElement};

pub use crate::error::{Error, Result};

pub mod animation;
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

pub fn select_element(selectors: &str) -> Result<Element> {
    document()?
        .query_selector(selectors)
        .map_err(|_| Error::InvalidSelectors(selectors.into()))?
        .ok_or_else(|| Error::ElementNotFound(selectors.into()))
}

pub fn select_element_cast<T: JsCast>(selectors: &str) -> Result<T> {
    select_element(selectors)?
        .dyn_into::<T>()
        .map_err(Error::ElementNotCast)
}

/// `setTimeout(func, delay_ms)` for a one-shot callback.
pub fn set_timeout(func: impl FnOnce() + 'static, delay_ms: i32) -> Result<i32> {
    let callback = Closure::once_into_js(func).unchecked_into();
    window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(&callback, delay_ms)
        .map_err(|_| Error::FailedToSetTimeout)
}
