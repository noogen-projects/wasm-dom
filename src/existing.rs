use wasm_bindgen::{UnwrapThrowExt, throw_str, JsCast, JsValue};
use web_sys::{Document, Window};
use js_sys::Reflect;

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

pub fn get_element_by_id<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| throw_str(&format!("Document should have an element with id = `{}`", id)))
        .dyn_into::<T>()
        .unwrap_or_else(|_| throw_str(&format!("Element with id = `{}` should cast to target type", id)))
}

pub fn select_element<T: JsCast>(selectors: &str) -> T {
    document()
        .query_selector(selectors)
        .unwrap_or_else(|value| throw_str(&format!("Specified selectors = `{}` is invalid: {:?}", selectors, value)))
        .unwrap_or_else(|| throw_str(&format!("Document should have an element accessible by selectors = `{}`", selectors)))
        .dyn_into::<T>()
        .unwrap_or_else(|element| throw_str(&format!("Element to select by `{}` should cast to target type: {:?}", selectors, element)))
}