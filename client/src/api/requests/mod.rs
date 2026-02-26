pub mod auth;
pub mod chunk;
pub mod file;
pub mod folder;
pub mod node;
pub mod share;

use crate::utils::auth::{get_token, go_to_login};
use crate::utils::browser::{LocalStorage, get_origin, get_window};
use crate::utils::error::{dyn_into, future_from_js_promise, wrap_js_err};
use anyhow::{Result, anyhow};
use leptos::wasm_bindgen::JsValue;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Display;
use web_sys::js_sys::{JsString, Uint8Array};
use web_sys::{Request, RequestInit, Response};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum RequestBody {
    Empty,
    Json(String),
    Bytes(Uint8Array),
}

async fn request(
    url_path: &str,
    method: RequestMethod,
    body: RequestBody,
    auth_token: Option<&String>,
    use_api_base_path: bool,
) -> Result<Response> {
    let opts = RequestInit::new();
    opts.set_method(&method.to_string());

    match &body {
        RequestBody::Empty => {}
        RequestBody::Json(json) => {
            opts.set_body(&JsValue::from_str(json));
        }
        RequestBody::Bytes(bytes) => {
            opts.set_body(bytes);
        }
    }

    let url = if use_api_base_path {
        let mut api_base_path: Option<String> = LocalStorage::get("api_base_path")?;

        // if the api_base_path is not set, use origin
        let origin = get_origin()?;

        let url = api_base_path.get_or_insert(origin);
        url.push_str(url_path);

        url.to_string()
    } else {
        url_path.to_string()
    };

    let request = wrap_js_err(Request::new_with_str_and_init(&url, &opts))?;

    match &body {
        RequestBody::Empty => {}
        RequestBody::Json(_) => {
            add_header(&request, ("Content-Type", "application/json"))?;
        }
        RequestBody::Bytes(_) => {
            add_header(&request, ("Content-Type", "application/octet-stream"))?;
        }
    }

    if let Some(auth_token) = auth_token {
        add_header(&request, ("authorization", &format!("Bearer {auth_token}")))?;
    }

    // obtain reference to the browser window object to use the request api
    let window = get_window()?;

    // the actual request
    let response_value: JsValue =
        future_from_js_promise(window.fetch_with_request(&request)).await?;
    let response: Response = dyn_into(response_value)?;

    Ok(response)
}

fn add_header(request: &Request, header: (&str, &str)) -> Result<()> {
    if let Err(js_error) = request.headers().set(header.0, header.1) {
        Err(anyhow!("could not add header: {:?}", js_error))
    } else {
        Ok(())
    }
}

//TODO use this for all api requests
async fn json_api_request<BodyT, ResponseT>(
    url: &str,
    request_method: RequestMethod,
    body: BodyT,
) -> Result<ResponseT>
where
    ResponseT: DeserializeOwned,
    BodyT: Serialize,
{
    let token = get_token()?;
    let token = Some(&token);

    let json = serde_json::to_string(&body)?;

    let body = if json == "null" {
        RequestBody::Empty
    } else {
        RequestBody::Json(json)
    };

    let response: Response = request(url, request_method, body, token, true).await?;

    // if the server returns Unauthorized, store the current url in the storage to be able to redirect to it on login
    if response.status() == 401 {
        go_to_login()?;
    } else {
        LocalStorage::unset("redirect_url")?;
    }

    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)
        .map_err(|_| anyhow!("could not parse json response: {:?}", response_string))?;
    Ok(response_object)
}

async fn string_from_response(response: Response) -> Result<String> {
    let text_promise = wrap_js_err(response.text())?;

    let string = future_from_js_promise::<JsString>(text_promise)
        .await?
        .as_string();

    if string.is_none() {
        return Err(anyhow!("response.text() is not a string? impossible"));
    }

    Ok(string.unwrap())
}

async fn uint8array_from_response(response: Response) -> Result<Uint8Array> {
    let array_buffer_promise = wrap_js_err(response.array_buffer())?;

    let bytes_arraybuffer = future_from_js_promise(array_buffer_promise).await?;
    let array = Uint8Array::new(&bytes_arraybuffer);
    Ok(array)
}

#[cfg(test)]
mod test {
    use crate::api::requests::{RequestBody, RequestMethod, request, string_from_response};
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct Post {
        pub userId: u64,
        pub id: u64,
        pub body: String,
        pub title: String,
    }

    #[wasm_bindgen_test]
    async fn test_request_1() {
        let url = "https://jsonplaceholder.typicode.com/posts";
        let method = RequestMethod::GET;
        let body = RequestBody::Empty;
        let auth_token = None;

        let response = request(url, method, body, auth_token, false).await.unwrap();
        assert_eq!(response.status(), 200);

        let response_text = string_from_response(response).await.unwrap();

        let _post_list: Vec<Post> = serde_json::from_str(&response_text).unwrap();
    }

    #[wasm_bindgen_test]
    async fn test_request_2() {
        let example_post = Post {
            userId: 1,
            id: 1,
            body: "rust".to_string(),
            title: "hello".to_string(),
        };

        let url = "https://jsonplaceholder.typicode.com/posts/1";
        let method = RequestMethod::PUT;
        let body = RequestBody::Json(serde_json::to_string(&example_post).unwrap());
        let auth_token = None;

        let response = request(url, method, body, auth_token, false).await.unwrap();
        assert_eq!(response.status(), 200);

        let response_text = string_from_response(response).await.unwrap();

        let post: Post = serde_json::from_str(&response_text).unwrap();

        assert_eq!(example_post, post);
    }

    #[test]
    fn test_display_request_method() {
        assert_eq!("GET", RequestMethod::GET.to_string());
    }
}
