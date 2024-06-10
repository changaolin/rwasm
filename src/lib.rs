use async_std::future::timeout;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn greet(name: &str) -> Result<JsValue, JsValue> {
    alert(&format!("Hello, {}!", name));

    let window = window().unwrap();
    let req: Request = Request::new_with_str_and_init(
        "https://api.github.com/users/rustwasm",
        &RequestInit::new(),
    )
    .unwrap();
    req.headers().set("Content-Type", "application").unwrap();
    req.headers().set("Accept", "application/lsjson").unwrap();
    req.headers().set("Accept-Language", "en-us").unwrap();
    let fetch = window.fetch_with_request(&req);

    let resp_value = match timeout(Duration::from_secs(5), JsFuture::from(fetch)).await {
        Ok(Ok(resp)) => resp,
        Ok(Err(e)) => {
            log(&format!("Fetch failed: {:?}", e));
            return Err(e.into());
        }
        Err(_) => {
            log("Fetch timed out");
            return Err(JsValue::from_str("Fetch timed out"));
        }
    };

    let resp: Response = match resp_value.dyn_into() {
        Ok(resp) => resp,
        Err(e) => {
            log(&format!("Response conversion failed: {:?}", e));
            return Err(e);
        }
    };

    let json = match timeout(Duration::from_secs(5), JsFuture::from(resp.json()?)).await {
        Ok(Ok(json)) => json,
        Ok(Err(e)) => {
            log(&format!("JSON conversion failed: {:?}", e));
            return Err(e.into());
        }
        Err(_) => {
            log("JSON conversion timed out");
            return Err(JsValue::from_str("JSON conversion timed out"));
        }
    };

    alert(&format!("Hello, {:?}!", json));
    Ok(json)
}
