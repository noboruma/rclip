use crate::{config, fail, TOKEN_PARAM};
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

#[mockable]
pub fn get_http_response_or_fail(url: &Url) -> HashMap<String, String> {
    reqwest::blocking::get(url.as_str())
        .unwrap_or_else(|_| fail(format!("GET {} failed", url.as_str()).as_str()))
        .json::<HashMap<String, String>>()
        .unwrap_or_else(|_| fail("GET result ill-formed"))
}

#[mockable]
pub fn get_http_or_fail(url: &Url) -> () {
    reqwest::blocking::get(url.as_str())
        .unwrap_or_else(|_| fail(format!("GET {} failed", url.as_str()).as_str()));
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
            config_path: PathBuf::from("/toor"),
            token: "".to_string(),
        };
        let url = prepare_endpoint(&config_context, "ok");
        assert_eq!(url.as_str(), "https://toto.com/ok?TOKEN=");
    }
    #[test]
    fn check_url_with_token() {
        let config_context = config::ConfigContext {
            base_url: url::Url::parse("https://toto.com").unwrap(),
            config_path: PathBuf::from("/toor"),
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
