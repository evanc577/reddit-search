use std::error::Error;
use std::fmt::{Debug, Display};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone, PartialEq)]
pub enum FetchError {
    InvalidJsValue { err: JsValue },
    BadStatus { code: u16 },
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJsValue { err } => write!(f, "invalid JS value: {:?}", err),
            Self::BadStatus { code } => write!(f, "received status code: {}", code),
        }
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self::InvalidJsValue { err: value }
    }
}

pub async fn fetch(url: String) -> Result<String, FetchError> {
    log::info!("Pushshift URL: {}", &url);
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    if !resp.ok() {
        return Err(FetchError::BadStatus {
            code: resp.status(),
        });
    }

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}
