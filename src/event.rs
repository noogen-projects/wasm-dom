use js_sys::Function;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{AddEventListenerOptions, Event, EventTarget};

#[inline]
pub fn closure(func: impl FnMut(Event) + 'static) -> Closure<dyn FnMut(Event)> {
    Closure::new(func)
}

pub fn js_function(func: impl FnMut(Event) + 'static) -> Function {
    closure(func).into_js_value().unchecked_into()
}

pub trait EventListener {
    fn add_steady_event_listener(&self, event: &str, handler: impl FnMut(Event) + 'static) -> Option<()>;
    fn add_steady_event_listener_with_options(
        &self,
        event: &str,
        handler: impl FnMut(Event) + 'static,
        options: &AddEventListenerOptions,
    ) -> Option<()>;

    #[must_use]
    fn add_manual_event_listener(
        &self,
        event: &str,
        handler: impl FnMut(Event) + 'static,
    ) -> Option<Closure<dyn FnMut(Event)>>;

    #[must_use]
    fn add_manual_event_listener_with_options(
        &self,
        event: &str,
        handler: impl FnMut(Event) + 'static,
        options: &AddEventListenerOptions,
    ) -> Option<Closure<dyn FnMut(Event)>>;
}

impl EventListener for EventTarget {
    fn add_steady_event_listener(&self, event: &str, handler: impl FnMut(Event) + 'static) -> Option<()> {
        let callback = js_function(handler);
        self.add_event_listener_with_callback(event, &callback).ok()
    }

    fn add_steady_event_listener_with_options(
        &self,
        event: &str,
        handler: impl FnMut(Event) + 'static,
        options: &AddEventListenerOptions,
    ) -> Option<()> {
        let callback = js_function(handler);
        self.add_event_listener_with_callback_and_add_event_listener_options(event, &callback, options)
            .ok()
    }

    fn add_manual_event_listener(
        &self,
        event: &str,
        handler: impl FnMut(Event) + 'static,
    ) -> Option<Closure<dyn FnMut(Event)>> {
        let callback = closure(handler);
        self.add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())
            .ok()
            .map(|_| callback)
    }

    fn add_manual_event_listener_with_options(
        &self,
        event: &str,
        handler: impl FnMut(Event) + 'static,
        options: &AddEventListenerOptions,
    ) -> Option<Closure<dyn FnMut(Event)>> {
        let callback = closure(handler);
        self.add_event_listener_with_callback_and_add_event_listener_options(
            event,
            callback.as_ref().unchecked_ref(),
            options,
        )
        .ok()
        .map(|_| callback)
    }
}
