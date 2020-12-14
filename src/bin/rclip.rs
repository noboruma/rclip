use remote_clipboard as rclip;
use std::env;
use std::io::{self, Read, Write};
use std::process;
use base64;

const HELP: &'static [u8] = br#"
    USAGE:
        rclip [-h | --help] [ARGS]

    FLAGS:
        -h, --help
                Prints help information

    ARGS:
        open
            Creates a new remote clipboard
        link [hash]
            Link current host with a remote clipboard
        copy [data]
            Copy the data to the remote clipboard
        paste
            Copy the data from the remote clipboard

    Please visit www.remote-clipboard.net for more information
"#;

const DISCLAIMER: &'static str = r#"variable is missing from the config file.
Please look at: https://github.com/noboruma/rclip-backends to setup your own secure backend
WARNING: rclip will use a default unsecured backend, your data might be accessed publicly
"#;

const HASH_SHORT_SIZE: usize = 6;
const LINK_TIMEOUT_SEC: usize = 30;

/// Util function to display and terminate the current process
/// with return code 1
fn fail(s: &str) -> ! {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    let _ = stderr.write(format!("{}{}{}", "error: ", s, "\n").as_bytes());
    let _ = stderr.flush();
    process::exit(1);
}

fn fail_error(res: &Result<(), rclip::ClipboardError>) {
    match res {
        Err(rclip::ClipboardError::BackendError) => fail("Failed to use clipboard with the backend (Is it opened/linked?)"),
        Err(rclip::ClipboardError::NetworkError(url)) => fail(("Failed to contact the backend: ".to_string()+&url).as_str()),
        Ok(_) => {},
    }
}

fn construct_default_config(stdout: &mut io::Stdout, config_path: std::path::PathBuf) -> rclip::config::ConfigContext {
    stdout.lock().write_all(format!("{} {}", rclip::URL_PARAM, DISCLAIMER).as_bytes()).unwrap();

    return rclip::config::ConfigContext {
        config_path,
        base_url: url::Url::parse(std::str::from_utf8(&base64::decode("aHR0cHM6Ly9hd3MucmVtb3RlLWNsaXBib2FyZC5uZXQ=").unwrap()).unwrap()).unwrap(),
        token: String::new(),
    };
}

fn construct_clipboard(stdout: &mut io::Stdout) -> rclip::Clipboard {

    let config = match rclip::config::ConfigContext::load() {
        Ok(config) => config,
        Err(rclip::config::Error::URLNotFound(path)) => construct_default_config(stdout, path),
        Err(rclip::config::Error::DirNotFound(s)) => fail((s+" not found").as_str()),
        Err(rclip::config::Error::URLIllFormed) => fail(format!("{}{}", rclip::URL_PARAM, " variable has ill-formed URL format").as_str()),
    };

    return rclip::Clipboard::from(config);
}

fn main() {
    let mut stdout = io::stdout();
    let stdin  = io::stdin();

    let mut args = env::args();
    if args.len() > 3 {
        fail("too many arguments");
    }

    match args.nth(1) {
        None => fail("not enough arguments"),
        Some(arg) => match arg.as_ref() {
            "open" => {
                let mut clipboard = construct_clipboard(&mut stdout);
                match args.next() {
                    Some(_) => fail("too many arguments"),
                    None => {
                        fail_error(&clipboard.open());
                        let config = &clipboard.config;
                        config.store();
                        stdout.lock()
                            .write(format!("Link with: {} (you have {} seconds)\n", &config.token[..HASH_SHORT_SIZE], LINK_TIMEOUT_SEC)
                                   .as_bytes()).unwrap();
                    }
                }
            },
            "link" => {
                let mut clipboard = construct_clipboard(&mut stdout);
                match args.next() {
                    Some(text) => {
                        fail_error(&clipboard.link(&mut text.as_bytes()));
                        clipboard.config.store();
                        stdout.lock().write("You are all set!\nYou can start using copy/paste\n".as_ref()).unwrap();
                    }
                    None => fail("not enough arguments"),
                }
            },
            "copy" => {
                let clipboard = construct_clipboard(&mut stdout);
                let mut input : Box<dyn Read> = match args.next() {
                    Some(text) => Box::new(rclip::stream::StringStream::from(text)),
                    None => Box::new(stdin.lock()),
                };
                fail_error(&clipboard.copy(&mut input));
            },
            "paste" => {
                let clipboard = construct_clipboard(&mut stdout);
                let _ = match args.next() {
                    Some(_) => fail("too many arguments"),
                    None => fail_error(&clipboard.paste(&mut stdout.lock())),
                };
            },
            "-h" | "--help" | _ => stdout.lock().write_all(HELP).unwrap(),
        },
    }

    stdout.flush().unwrap();
    process::exit(0);
}
