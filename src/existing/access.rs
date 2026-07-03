use std::fmt;

use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt, throw_str};
use web_sys::{Element, EventTarget, HtmlElement, Node};

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

pub trait Cast {
    fn cast<T: JsCast>(self) -> T;
    fn maybe_cast<T: JsCast>(self) -> Option<T>;

    fn cast_as<T: JsCast>(&self) -> &T;
    fn maybe_cast_as<T: JsCast>(&self) -> Option<&T>;
}

impl<E: JsCast + fmt::Debug> Cast for E {
    fn cast<T: JsCast>(self) -> T {
        self.dyn_into::<T>()
            .unwrap_or_else(|element| throw_str(&format!("{element:?} should cast to target type")))
    }

    fn maybe_cast<T: JsCast>(self) -> Option<T> {
        self.dyn_into::<T>().ok()
    }

    fn cast_as<T: JsCast>(&self) -> &T {
        self.dyn_ref::<T>()
            .unwrap_or_else(|| throw_str(&format!("{self:?} should cast to target type")))
    }

    fn maybe_cast_as<T: JsCast>(&self) -> Option<&T> {
        self.dyn_ref::<T>()
    }
}

pub trait CastToElement {
    fn into_element(self) -> Element;
    fn maybe_into_element(self) -> Option<Element>;

    fn as_element(&self) -> &Element;
    fn maybe_as_element(&self) -> Option<&Element>;
}

impl CastToElement for EventTarget {
    fn into_element(self) -> Element {
        self.cast()
    }

    fn maybe_into_element(self) -> Option<Element> {
        self.maybe_cast()
    }

    fn as_element(&self) -> &Element {
        self.cast_as()
    }

    fn maybe_as_element(&self) -> Option<&Element> {
        self.maybe_cast_as()
    }
}

impl CastToElement for Node {
    fn into_element(self) -> Element {
        self.cast()
    }

    fn maybe_into_element(self) -> Option<Element> {
        self.maybe_cast()
    }

    fn as_element(&self) -> &Element {
        self.cast_as()
    }

    fn maybe_as_element(&self) -> Option<&Element> {
        self.maybe_cast_as()
    }
}

pub trait CastToHtmlElement {
    fn into_html(self) -> HtmlElement;
    fn maybe_into_html(self) -> Option<HtmlElement>;

    fn as_html(&self) -> &HtmlElement;
    fn maybe_as_html(&self) -> Option<&HtmlElement>;
}

impl CastToHtmlElement for Element {
    fn into_html(self) -> HtmlElement {
        self.cast()
    }

    fn maybe_into_html(self) -> Option<HtmlElement> {
        self.maybe_cast()
    }

    fn as_html(&self) -> &HtmlElement {
        self.cast_as()
    }

    fn maybe_as_html(&self) -> Option<&HtmlElement> {
        self.maybe_cast_as()
    }
}
