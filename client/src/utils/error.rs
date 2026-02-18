use anyhow::{Error, anyhow};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

/// Wrap a js value (should be an error) into `anyhow::Error` type. The function attempts to
/// automatically cast to `DomException` and only falls back to standard error handling.
pub fn wrap_js_err<T>(result: Result<T, JsValue>) -> Result<T, Error> {
    result.map_err(|js_value| {
        if let Some(dom_exception) = js_value.dyn_ref::<web_sys::DomException>() {
            return anyhow!("{}: {}", dom_exception.name(), dom_exception.message());
        }

        anyhow!("{:?}", js_value)
    })
}

pub async fn future_from_js_promise<T: JsCast>(promise: Promise) -> Result<T, Error> {
    let js_value = wrap_js_err(JsFuture::from(promise).await)?;
    dyn_into(js_value)
}

pub fn dyn_into<T: JsCast>(js_value: JsValue) -> Result<T, Error> {
    wrap_js_err(js_value.dyn_into::<T>())
}
