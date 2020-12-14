use crate::{config, TOKEN_PARAM, ClipboardError};
use std::collections::HashMap;
use std::io::Read;
use url::Url;

use mocktopus::macros::*;

pub fn prepare_endpoint(config_context: &config::ConfigContext, path: &str) -> Url {
    let mut url = config_context.base_url.clone();
    url.set_path(path);
    append_query(&mut url, &mut config_context.token.as_bytes(), &TOKEN_PARAM);
    return url;
}

#[cfg(not(target_arch = "wasm32"))]
#[mockable]
pub fn get_http(url: &Url) -> Result<(), ClipboardError> {
    match reqwest::blocking::get(url.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => return Err(ClipboardError::NetworkError(url.to_string())),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[mockable]
pub fn get_http_response(url: &Url) -> Result<HashMap<String, String>, ClipboardError>{
    let resp = reqwest::blocking::get(url.as_str());
    let resp = match resp {
        Ok(resp) => resp,
        Err(_) => return Err(ClipboardError::NetworkError(url.to_string())),
    };
    return match resp .json::<HashMap<String, String>>() {
        Ok(resp) => Ok(resp),
        Err(_) => return Err(ClipboardError::BackendError),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F: futures::Future>(completion : F) {
    futures::executor::block_on(completion);
}

#[cfg(target_arch = "wasm32")]
pub fn execute<F: 'static + futures::Future<Output = ()>>(completion : F) {
    use wasm_bindgen_futures::spawn_local;
    spawn_local(completion);
}

#[mockable]
pub fn get_http_response_comp(url: &Url, completion: Box<dyn Fn(Result<HashMap<String, String>, ClipboardError>)>) {

    let boxed = Box::from(url.clone());
    execute(async move {
        let resp = match (reqwest::get(boxed.as_str())).await {
            Ok(resp) => resp,
            Err(_) => return completion(Err(ClipboardError::NetworkError(boxed.to_string()))),
        };
        let resp = match resp.json::<HashMap<String, String>>().await {
            Ok(resp) => Ok(resp),
            Err(_) => Err(ClipboardError::BackendError),
        };
        completion(resp);
    });
}

#[cfg(target_arch = "wasm32")]
#[mockable]
pub fn get_http(url: &Url) -> Result<(), ClipboardError> {
    use wasm_bindgen_futures::spawn_local;

    let boxed = Box::from(url.clone());
    spawn_local(async move {
        let _db = (reqwest::get(boxed.as_str())).await;
    });
    Ok(())
}

pub fn append_query(url: &mut Url, input: &mut dyn Read, param: &str) {
    let mut query = String::from("");
    input.read_to_string(&mut query).unwrap();
    url.query_pairs_mut().append_pair(&param, query.as_str());
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config;
    use std::path::PathBuf;

    #[test]
    fn check_url_without_token() {
        let config_context = config::ConfigContext {
            base_url: url::Url::parse("https://toto.com").unwrap(),
            config_path: PathBuf::from("/foo"),
            token: "".to_string(),
        };
        let url = prepare_endpoint(&config_context, "ok");
        assert_eq!(url.as_str(), "https://toto.com/ok?TOKEN=");
    }

    #[test]
    fn check_url_with_token() {
        let config_context = config::ConfigContext {
            base_url: url::Url::parse("https://toto.com").unwrap(),
            config_path: PathBuf::from("/foo"),
            token: "abc".to_string(),
        };
        let url = prepare_endpoint(&config_context, "ok");
        assert_eq!(url.as_str(), "https://toto.com/ok?TOKEN=abc");
    }

    #[test]
    fn check_url_with_text() {
        let mut url = url::Url::parse("https://toto.com/?aa=a").unwrap();
        append_query(&mut url, &mut "ok".as_bytes(), &"text");
        assert_eq!(url.as_str(), "https://toto.com/?aa=a&text=ok");
    }
}
