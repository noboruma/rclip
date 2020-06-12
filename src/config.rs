use crate::{fail, URL_PARAM, TOKEN_PARAM};
use dotenv;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

use mocktopus::macros::*;

pub struct ConfigContext {
    pub config_path: PathBuf,
    pub base_url: url::Url,
    pub token: String,
}

fn get_config_path() -> std::path::PathBuf {
    let mut config_path = dirs::home_dir()
        .unwrap_or_else(|| fail("$HOME dir not defined"));
    config_path.push(".rclip.env");
    return config_path;
}

#[mockable]
pub fn load_config() -> ConfigContext {
    let config_path = get_config_path();
    dotenv::from_path(config_path.as_path())
        .unwrap_or_else(|_| fail(format!("{} file is missing",
                    config_path.to_str().unwrap()).as_str()));

    let url = Url::parse(&dotenv::var(URL_PARAM)
        .unwrap_or_else(|_| fail("URL variable is missing from the config file\nPlease look at: https://github.com/noboruma/rclip-backends to setup a backend")))
        .unwrap_or_else(|_| fail("URL variable has an ill-formed URL format"));

    let token = dotenv::var(TOKEN_PARAM)
        .unwrap_or(String::new());

    return ConfigContext {
        config_path,
        base_url: url,
        token,
    };
}

#[mockable]
pub fn store_config(config_context: &ConfigContext, token: &String) {
    let mut data = String::new();
    data += URL_PARAM; data += "="; data += config_context.base_url.as_str(); data += "\n";
    data += TOKEN_PARAM; data += "="; data += token.as_str(); data += "\n";

    let mut config_file = File::create(&config_context.config_path).unwrap();
    config_file.write(data.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_config_path_exists() {
        assert!(!get_config_path().into_os_string().is_empty());
    }
}
