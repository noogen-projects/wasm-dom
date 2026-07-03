use futures::channel::oneshot;
use js_sys::Function;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

use crate::{Error, Result};

pub fn request_frame(callback: &Function) -> Result<i32> {
    super::window()?
        .request_animation_frame(callback)
        .map_err(|_| Error::FailedToRequestAnimationFrame)
}

/// Awaits a single animation frame, like `await new Promise(r => requestAnimationFrame(r))`.
pub async fn next_frame() -> Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let callback = Closure::once_into_js(move |_ts: f64| {
        tx.send(()).ok();
    });
    request_frame(callback.unchecked_ref())?;

    rx.await.map_err(|_| Error::FailedToWaitNextAnimationFrame)
}
