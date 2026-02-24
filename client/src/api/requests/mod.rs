pub mod auth;
pub mod chunk;
pub mod file;
pub mod folder;
pub mod node;
pub mod share;

use crate::constants::API_BASE_PATH;
use crate::utils::auth::get_token;
use crate::utils::browser::get_window;
use crate::utils::error::{dyn_into, future_from_js_promise, wrap_js_err};
use anyhow::{Result, anyhow};
use leptos::wasm_bindgen::JsValue;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Display;
use web_sys::js_sys::{JsString, Uint8Array};
use web_sys::{Request, RequestInit, Response, Url};

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
    url: String,
    method: RequestMethod,
    body: RequestBody,
    //TODO maybe remove query parameters as they are never used
    query_parameters: Vec<(String, String)>,
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

    let url = wrap_js_err(Url::new(&if use_api_base_path {
        let mut url_string = API_BASE_PATH.to_string();
        url_string.push_str(&url);
        url_string
    } else {
        url
    }))?;

    for (key, value) in query_parameters {
        url.search_params().append(&key, &value);
    }

    let request = wrap_js_err(Request::new_with_str_and_init(
        &url.to_string().as_string().unwrap(),
        &opts,
    ))?;

    match &body {
        RequestBody::Empty => {}
        RequestBody::Json(_) => {
            if let Err(js_error) = request.headers().set("Content-Type", "application/json") {
                return Err(anyhow!("could not add header: {:?}", js_error));
            };
        }
        RequestBody::Bytes(_) => {
            if let Err(js_error) = request
                .headers()
                .set("Content-Type", "application/octet-stream")
            {
                return Err(anyhow!("could not add header: {:?}", js_error));
            };
        }
    }

    if let Some(auth_token) = auth_token {
        if let Err(js_error) = request
            .headers()
            .set("authorization", &format!("Bearer {auth_token}"))
        {
            return Err(anyhow!("could not add header: {:?}", js_error));
        };
    }

    // obtain reference to the browser window object to use the request api
    let window = get_window()?;

    // the actual request
    let response_value: JsValue =
        future_from_js_promise(window.fetch_with_request(&request)).await?;
    let response: Response = dyn_into(response_value)?;

    //TODO maybe here we should redirect to the login page in case of a 403
    //TODO maybe get the token from storage here to avoid duplicate code

    Ok(response)
}

async fn json_api_request<BodyT, ResponseT>(
    url: String,
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

    let response: Response = request(
        url,
        request_method,
        RequestBody::Json(json),
        vec![],
        token,
        true,
    )
    .await?;

    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string).map_err(|_| anyhow!(
            "could not parse json response: {:?}",
            response_string
        ))?;
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
        let query_parameters = vec![];
        let auth_token = None;

        let response = request(
            url.to_string(),
            method,
            body,
            query_parameters,
            auth_token,
            false,
        )
        .await
        .unwrap();
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
        let query_parameters = vec![];
        let auth_token = None;

        let response = request(
            url.to_string(),
            method,
            body,
            query_parameters,
            auth_token,
            false,
        )
        .await
        .unwrap();
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
