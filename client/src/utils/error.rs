use anyhow::{anyhow, Error};
use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;


/// Wrap a js value (should be an error) into an anyhow::Error type
pub fn wrap_js_err<T>(result: Result<T, JsValue>) -> Result<T, Error> {
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            Err(anyhow!("{:?}", err))
        }
    }
}

pub async fn future_from_js_promise<T: JsCast> (promise: Promise) -> Result<T, Error> {
    let js_value = wrap_js_err(JsFuture::from(promise).await)?;
    dyn_into(js_value)
}

pub fn dyn_into<T: JsCast> (js_value: JsValue) -> Result<T, Error> {
    wrap_js_err(js_value.dyn_into::<T>())
}