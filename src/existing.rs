use js_sys::Reflect;
use wasm_bindgen::{throw_str, JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Document, Location, Window};

pub trait JsObjectAccess {
    fn get(&self, property: impl Into<JsValue>) -> JsValue;
    fn set(&self, property: impl Into<JsValue>, value: impl Into<JsValue>) -> bool;
}

impl JsObjectAccess for JsValue {
    fn get(&self, property: impl Into<JsValue>) -> JsValue {
        Reflect::get(self, &property.into()).expect_throw("Target should be an Object")
    }

    fn set(&self, property: impl Into<JsValue>, value: impl Into<JsValue>) -> bool {
        Reflect::set(self, &property.into(), &value.into()).expect_throw("Target should be an Object")
    }
}

pub fn window() -> Window {
    super::window().expect_throw("Should have a window in this context")
}

pub fn document() -> Document {
    window().document().expect_throw("Window should have a document")
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

pub fn select_element<T: JsCast>(selectors: &str) -> T {
    document()
        .query_selector(selectors)
        .unwrap_or_else(|value| throw_str(&format!("Specified selectors = `{selectors}` is invalid: {value:?}")))
        .unwrap_or_else(|| {
            throw_str(&format!(
                "Document should have an element accessible by selectors = `{selectors}`"
            ))
        })
        .dyn_into::<T>()
        .unwrap_or_else(|element| {
            throw_str(&format!(
                "Element to select by `{selectors}` should cast to target type: {element:?}"
            ))
        })
}
