use std::io::{Write, Read};

const TEXT_PARAM: &'static str = "text";
const TOKEN_PARAM: &'static str = "TOKEN";
const SHORTHASH_PARAM: &'static str = "shortHash";
pub const URL_PARAM: &'static str = "RCLIP_URL";
const OPEN_ENDPOINT: &'static str = "open";
const LINK_ENDPONT: &'static str = "link";
const COPY_ENDPOINT: &'static str = "push";
const PASTE_ENDPONT: &'static str = "pull";

pub mod config;
pub mod stream;
mod http;

#[cfg(target_arch = "wasm32")]
pub mod js;

#[derive(Debug)]
pub enum ClipboardError {
    NetworkError(String),
    BackendError,
}

pub struct Clipboard {
    pub config: config::ConfigContext,
}

impl Clipboard {

    pub fn from(config: config::ConfigContext) -> Clipboard {
        Clipboard {
            config: config,
        }
    }

    /// Push readable data into the remote clipboard
    ///
    /// # Arguments
    ///
    /// * `input` - A readable object
    ///
    /// # Example
    ///
    /// ```
    /// use remote_clipboard as rclip;
    /// use std::path::PathBuf;
    /// use url::Url;
    /// let clipboard = rclip::Clipboard::from(rclip::config::ConfigContext {
    ///     config_path: PathBuf::from("/blah"),
    ///     base_url: url::Url::parse("https://toto.com").unwrap(),
    ///     token: String::from("token"),
    /// });
    /// clipboard.copy(&mut "toto".as_bytes());
    /// ```
    pub fn copy(&self, input: &mut dyn Read) -> Result<(), ClipboardError> {
        let mut url = http::prepare_endpoint(&self.config, COPY_ENDPOINT);
        http::append_query(&mut url, input, &TEXT_PARAM);
        http::get_http(&url)
    }

    /// Pull data from the remote clipboard
    /// # Example
    ///
    /// ```
    /// use remote_clipboard as rclip;
    /// use std::io::{self};
    /// use std::path::PathBuf;
    /// use url::Url;
    /// let clipboard = rclip::Clipboard::from(rclip::config::ConfigContext {
    ///     config_path: PathBuf::from("/blah"),
    ///     base_url: url::Url::parse("https://toto.com").unwrap(),
    ///     token: String::from("token"),
    /// });
    /// let stdout = io::stdout();
    /// //clipboard.paste(&mut stdout.lock());
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn paste(&self, output: &mut dyn Write) -> Result<(), ClipboardError> {
        let url = http::prepare_endpoint(&self.config, PASTE_ENDPONT);
        let resp = http::get_http_response(&url)?;
        let _ = match resp.get(&String::from(TEXT_PARAM)) {
            Some(number) => output.write(number.as_ref()),
            _ => output.write(b""),
        };
        Ok(())
    }

    pub fn paste_comp(&self, completion: Box<dyn Fn(Result<String, ClipboardError>)>) -> Result<(), ClipboardError> {

        use std::collections::HashMap;
        let wrapper = move |resp : Result<HashMap<String, String>, ClipboardError>| {
            match resp {
                Ok(resp) => {
                    match resp.get(&String::from(TEXT_PARAM)) {
                        Some(x) =>  {
                            return completion(Ok(x.to_string()));
                        },
                        None => (),
                    };
                },
                _ => (),
            };
            completion(Err(ClipboardError::BackendError));
        };

        let url = http::prepare_endpoint(&self.config, PASTE_ENDPONT);
        http::get_http_response_comp(&url, Box::new(wrapper));
        Ok(())
    }

    /// Open a new remote clipboard
    /// And generate new token if needed
    /// The function is synchronous
    /// # Example
    ///
    /// ```
    /// use remote_clipboard as rclip;
    /// use std::path::PathBuf;
    /// use url::Url;
    /// let mut clipboard = rclip::Clipboard::from(rclip::config::ConfigContext {
    ///     config_path: PathBuf::from("/blah"),
    ///     base_url: url::Url::parse("https://toto.com").unwrap(),
    ///     token: String::from("token"),
    /// });
    /// //clipboard.open();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(&mut self) -> Result<(), ClipboardError> {
        let url = http::prepare_endpoint(&self.config, OPEN_ENDPOINT);
        let resp = http::get_http_response(&url)?;
        match resp.get(&String::from(TEXT_PARAM)) {
            Some(token) => { self.config.token = token.clone(); Ok(()) },
            _ => Err(ClipboardError::BackendError)
        }
    }

    pub fn open_comp(&self, completion: Box<dyn Fn(Result<String, ClipboardError>)>) -> Result<(), ClipboardError> {

        use std::collections::HashMap;
        let wrapper = move |resp : Result<HashMap<String, String>, ClipboardError>| {
            match resp {
                Ok(resp) => {
                    match resp.get(&String::from(TEXT_PARAM)) {
                        Some(x) =>  {
                            return completion(Ok(x.to_string()));
                        },
                        None => (),
                    };
                },
                _ => (),
            };
            completion(Err(ClipboardError::BackendError));
        };

        let url = http::prepare_endpoint(&self.config, OPEN_ENDPOINT);
        http::get_http_response_comp(&url, Box::new(wrapper));
        Ok(())
    }

    /// Link against newly opened remote clipboard
    ///
    /// # Arguments
    ///
    /// * `input` - A readable object containing the short living hash
    ///
    /// # Example
    ///
    /// ```
    /// use remote_clipboard as rclip;
    /// use std::io::{self};
    /// use std::path::PathBuf;
    /// use url::Url;
    /// let mut clipboard = rclip::Clipboard::from(rclip::config::ConfigContext {
    ///     config_path: PathBuf::from("/blah"),
    ///     base_url: url::Url::parse("https://toto.com").unwrap(),
    ///     token: String::from("token"),
    /// });
    /// //clipboard.link(&mut "012345".as_bytes());
    /// ```
    /// TODO: check input size
    #[cfg(not(target_arch = "wasm32"))]
    pub fn link(&mut self, input: &mut dyn Read) -> Result<(), ClipboardError> {
        let mut url = http::prepare_endpoint(&self.config, LINK_ENDPONT);
        http::append_query(&mut url, input, SHORTHASH_PARAM);
        let resp = http::get_http_response(&url)?;
        match resp.get(&String::from(TOKEN_PARAM)) {
            Some(token) => {self.config.token = token.clone(); Ok(())},
            _ => Err(ClipboardError::BackendError),
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::config::ConfigContext;
    use mocktopus::mocking::*;
    use std::path::PathBuf;
    use std::io::{self};
    use super::*;

    fn mocked_config() -> ConfigContext {
        ConfigContext {
            config_path: PathBuf::from("/blah"),
            base_url: url::Url::parse("https://toto.com").unwrap(),
            token: String::from("token"),
        }
    }

    #[test]
    fn check_push() {
        http::get_http.mock_safe(|_url| MockResult::Return(Ok(())));
        let clipboard = Clipboard::from(mocked_config());

        let _ = clipboard.copy(&mut "toto".as_bytes());
    }

    #[test]
    fn check_pull() {
        http::get_http_response.mock_safe(|_url|
            MockResult::Return(Ok(vec![(TEXT_PARAM.to_string(), "stuff".to_string())]
                .into_iter().collect())));
        let clipboard = Clipboard::from(mocked_config());
        let stdout = io::stdout();

        let _ = clipboard.paste(&mut stdout.lock());
    }

    #[test]
    fn check_open() {
        http::get_http_response.mock_safe(|_url|
            MockResult::Return(Ok(vec![(TEXT_PARAM.to_string(), "012345".to_string())]
                .into_iter().collect())));
        let mut clipboard = Clipboard::from(mocked_config());

        let _ = clipboard.open();
    }

    #[test]
    #[should_panic]
    fn check_open_panic() {
        http::get_http_response.mock_safe(|_url|
            MockResult::Return(Ok(vec![("nonexist".to_string(), "01234".to_string())]
                .into_iter().collect())));
        let mut clipboard = Clipboard::from(mocked_config());

        clipboard.open().unwrap();
    }

    #[test]
    fn check_link() {
        http::get_http_response.mock_safe(|_url|
            MockResult::Return(Ok(vec![(TOKEN_PARAM.to_string(), "stuff".to_string())].into_iter().collect())));
        let mut clipboard = Clipboard::from(mocked_config());

        let _ = clipboard.link(&mut "toto".as_bytes());
    }

}
