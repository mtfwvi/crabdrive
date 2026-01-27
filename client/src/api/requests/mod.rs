pub mod chunk;
pub mod file;
pub mod folder;
pub mod node;

use crate::constants::API_BASE_PATH;
use leptos::wasm_bindgen::JsValue;
use std::fmt::Display;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
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
    query_parameters: Vec<(String, String)>,
    auth_token: Option<&String>,
    use_api_base_path: bool,
) -> Result<Response, JsValue> {
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

    let url = Url::new(&if use_api_base_path {
        let mut url_string = API_BASE_PATH.to_string();
        url_string.push_str(&url);
        url_string
    } else {
        url
    })?;

    for (key, value) in query_parameters {
        url.search_params().append(&key, &value);
    }

    let request = Request::new_with_str_and_init(&url.to_string().as_string().unwrap(), &opts)?;

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
