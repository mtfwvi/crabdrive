pub mod chunk;
pub mod file;
pub mod folder;
pub mod node;

use leptos::wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::{Blob, Request, RequestInit, Response, Url};

#[allow(clippy::upper_case_acronyms)]
enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

enum RequestBody {
    Empty,
    Json(String),
    Bytes(Uint8Array),
}

async fn request(
    url: String,
    method: RequestMethod,
    body: RequestBody,
    query_parameters: Vec<(String, String)>,
    auth_token: Option<&String>,
) -> Result<Response, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(match method {
        RequestMethod::GET => "GET",
        RequestMethod::POST => "POST",
        RequestMethod::PATCH => "PATCH",
        RequestMethod::DELETE => "DELETE",
        RequestMethod::PUT => "PUT",
    });

    match &body {
        RequestBody::Empty => {}
        RequestBody::Json(json) => {
            opts.set_body(&JsValue::from_str(json));
        }
        RequestBody::Bytes(bytes) => {
            opts.set_body(bytes);
        }
    }

    let url = Url::new(&url)?;

    for (key, value) in query_parameters {
        url.search_params().append(&key, &value);
    }

    let request = Request::new_with_str_and_init(&url.as_string().unwrap(), &opts)?;

    match &body {
        RequestBody::Empty => {}
        RequestBody::Json(_) => {
            request.headers().set("Content-Type", "application/json")?;
        }
        RequestBody::Bytes(_) => {
            request
                .headers()
                .set("Content-Type", "application/octet-stream")?;
        }
    }

    if let Some(auth_token) = auth_token {
        request
            .headers()
            .set("authorization", &format!("Bearer {auth_token}"))?;
    }

    // obtain reference to the browser window object to use the request api
    let window = web_sys::window().unwrap();

    // the actual request
    let response_value: JsValue = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into::<Response>()?;

    //TODO maybe here we should redirect to the login page in case of a 403

    Ok(response)
}

async fn string_from_response(response: Response) -> Result<String, JsValue> {
    let string = JsFuture::from(response.text()?).await?.as_string().unwrap();
    Ok(string)
}

async fn uint8array_from_response(response: Response) -> Result<Uint8Array, JsValue> {
    let bytes_arraybuffer = JsFuture::from(response.array_buffer()?).await?;
    let array = Uint8Array::new(&bytes_arraybuffer);
    Ok(array)
}
