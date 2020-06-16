use std::io::{self, Write, Read};
use std::process;

const TEXT_PARAM: &'static str = "text";
const TOKEN_PARAM: &'static str = "TOKEN";
const SHORTHASH_PARAM: &'static str = "shortHash";
const URL_PARAM: &'static str = "URL";
const OPEN_ENDPOINT: &'static str = "dev/open";
const LINK_ENDPONT: &'static str = "dev/link";
const COPY_ENDPOINT: &'static str = "dev/push";
const PASTE_ENDPONT: &'static str = "dev/pull";
const HASH_SHORT_SIZE: usize = 6;

mod config;
mod http;

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
/// //rclip::push(&mut "toto".as_bytes());
/// ```
pub fn push(input: &mut dyn Read) {
    let config_context = config::load_config();
    if config_context.token.is_empty() {
        fail("No clipboard linked, please link or open a new clipboard");
    }

    let mut url = http::prepare_endpoint(&config_context, COPY_ENDPOINT);
    http::append_query(&mut url, input, &TEXT_PARAM);
    http::get_http_or_fail(&url);
}

/// Pull data from the remote clipboard
/// # Example
///
/// ```
/// use remote_clipboard as rclip;
/// use std::io::{self};
/// let stdout = io::stdout();
/// //rclip::pull(stdout);
/// ```
pub fn pull(stdout: io::Stdout) {
    let config_context = config::load_config();
    if config_context.token.is_empty() {
        fail("No clipboard linked, please link or open a new clipboard");
    }

    let url = http::prepare_endpoint(&config_context, PASTE_ENDPONT);
    let resp = http::get_http_response_or_fail(&url);
    let _ = match resp.get(&String::from(TEXT_PARAM)) {
        Some(number) => stdout.lock().write(number.as_ref()),
        _ => stdout.lock().write(b""),
    };
}

/// Open a new remote clipboard
/// and display a short living hash for linking
/// another client
/// # Example
///
/// ```
/// use remote_clipboard as rclip;
/// use std::io::{self};
/// let stdout = io::stdout();
/// //rclip::open(stdout);
/// ```
pub fn open(stdout: io::Stdout) {
    let config_context = config::load_config();

    let url = http::prepare_endpoint(&config_context, OPEN_ENDPOINT);
    let resp = http::get_http_response_or_fail(&url);
    let _ = match resp.get(&String::from(TEXT_PARAM)) {
        Some(token) => {
            config::store_config(&config_context, &token);
            stdout.lock().write(&token.as_bytes()[..HASH_SHORT_SIZE])
        },
        _ => fail("Failed to open new link with back-end"),
    };
}

/// Link against newly opened remote clipboard
///
/// # Arguments
///
/// * `stdout`
/// * `input` - A readable object containing the short living hash
///
/// # Example
///
/// ```
/// use remote_clipboard as rclip;
/// use std::io::{self};
/// let stdout = io::stdout();
/// //rclip::link(stdout, &mut "012345".as_bytes());
/// ```
pub fn link(stdout: io::Stdout, input: &mut dyn Read) {
    let config_context = config::load_config();

    let mut url = http::prepare_endpoint(&config_context, LINK_ENDPONT);
    http::append_query(&mut url, input, SHORTHASH_PARAM);
    let resp = http::get_http_response_or_fail(&url);
    let _ = match resp.get(&String::from(TOKEN_PARAM)) {
        Some(token) => {
            config::store_config(&config_context, &token);
            stdout.lock().write("You are all set!\nYou can start using push/pull\n".as_ref())
        },
        _ => fail("Failed to link with the back-end"),
    };
}

/// Util function to display and terminate the current process
/// with return code 1
pub fn fail(s: &str) -> ! {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    let _ = stderr.write(format!("{}{}{}", "error: ", s, "\n").as_bytes());
    let _ = stderr.flush();
    process::exit(1);
}

#[cfg(test)]
mod tests {

    use crate::config::ConfigContext;
    use mocktopus::mocking::*;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn check_push() {
        http::get_http_or_fail.mock_safe(|_url| MockResult::Return(()));
        config::load_config.mock_safe(|| MockResult::Return(ConfigContext {
            config_path: PathBuf::from("/blah"),
            base_url: url::Url::parse("https://toto.com").unwrap(),
            token: String::from("token"),
        }));
        push(&mut "toto".as_bytes());
    }

    #[test]
    fn check_pull() {
        http::get_http_response_or_fail.mock_safe(|_url|
            MockResult::Return(vec![(TEXT_PARAM.to_string(), "stuff".to_string())].into_iter().collect()));
        config::load_config.mock_safe(|| MockResult::Return(ConfigContext {
            config_path: PathBuf::from("/blah"),
            base_url: url::Url::parse("https://toto.com").unwrap(),
            token: String::from("token"),
        }));
        pull(io::stdout());
    }

    #[test]
    fn check_open() {
        http::get_http_response_or_fail.mock_safe(|_url|
            MockResult::Return(vec![(TEXT_PARAM.to_string(), "012345".to_string())].into_iter().collect()));
        config::load_config.mock_safe(|| MockResult::Return(ConfigContext {
            config_path: PathBuf::from("/blah"),
            base_url: url::Url::parse("https://toto.com").unwrap(),
            token: String::from("token"),
        }));
        config::store_config.mock_safe(|_, _| MockResult::Return(()));

        open(io::stdout());
    }

    #[test]
    #[should_panic]
    fn check_open_panic() {
        http::get_http_response_or_fail.mock_safe(|_url|
            MockResult::Return(vec![(TEXT_PARAM.to_string(), "01234".to_string())].into_iter().collect()));
        config::load_config.mock_safe(|| MockResult::Return(ConfigContext {
            config_path: PathBuf::from("/blah"),
            base_url: url::Url::parse("https://toto.com").unwrap(),
            token: String::from("token"),
        }));
        config::store_config.mock_safe(|_, _| MockResult::Return(()));

        open(io::stdout());
    }

    #[test]
    fn check_link() {
        http::get_http_response_or_fail.mock_safe(|_url|
            MockResult::Return(vec![(TOKEN_PARAM.to_string(), "stuff".to_string())].into_iter().collect()));
        config::load_config.mock_safe(|| MockResult::Return(ConfigContext {
            config_path: PathBuf::from("/blah"),
            base_url: url::Url::parse("https://toto.com").unwrap(),
            token: String::from("token"),
        }));
        config::store_config.mock_safe(|_, _| MockResult::Return(()));

        link(io::stdout(), &mut "toto".as_bytes());
    }

}
