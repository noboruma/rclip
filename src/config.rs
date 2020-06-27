use crate::{URL_PARAM, TOKEN_PARAM};
use dotenv;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub enum Error {
    DirNotFound(String),
    URLNotFound(PathBuf),
    URLIllFormed,
}

#[derive(Clone)]
pub struct ConfigContext {
    pub config_path: PathBuf,
    pub base_url: url::Url,
    pub token: String,
}

fn get_config_path() -> Option<std::path::PathBuf> {
    let mut config_path = match dirs::home_dir() {
        Some(resp) => resp,
        None => return None,
    };
    config_path.push(".rclip.env");
    return Some(config_path);
}

impl ConfigContext {

    pub fn load() -> Result<ConfigContext, Error> {
        let config_path = match get_config_path() {
            Some(path) => path,
            None => return Err(Error::DirNotFound("$HOME not found".to_string())),
        };

        match dotenv::from_path(config_path.as_path()) {
            Ok(()) => (),
            Err(_) => (),
        };

        let url = match dotenv::var(URL_PARAM) {
            Ok(resp) => resp,
            Err(_) => return Err(Error::URLNotFound(config_path)),
        };

        let url = match Url::parse(&url) {
            Ok(resp) => resp,
            Err(_) => return Err(Error::URLIllFormed),
        };

        let token = dotenv::var(TOKEN_PARAM)
            .unwrap_or(String::new());

        return Ok(ConfigContext {
            config_path,
            base_url: url,
            token,
        });
    }

    pub fn store(&self) {
        let mut data = String::new();
        data += URL_PARAM; data += "="; data += self.base_url.as_str(); data += "\n";
        data += TOKEN_PARAM; data += "="; data += self.token.as_str(); data += "\n";

        let mut config_file = File::create(&self.config_path).unwrap();
        config_file.write(data.as_bytes()).unwrap();
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_config_path_exists() {
        assert!(!get_config_path().unwrap().into_os_string().is_empty());
    }
}
