use remote_clipboard as rclip;
use std::env;
use std::io::{self, Read, Write};
use std::process;

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
        push [data]
            Copy the data to the remote clipboard
        pull
            Copy the data from the remote clipboard

"#;

const HASH_SHORT_SIZE: usize = 6;

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
        Err(rclip::ClipboardError::BackendError) => fail("Failed to open clipboard with the back-end"),
        Err(rclip::ClipboardError::NetworkError(url)) => fail(("Failed to contact the back-end at: ".to_string()+&url).as_str()),
        _ => {},
    }
}

fn main() {

    let stdout = io::stdout();
    let stdin  = io::stdin();

    let mut args = env::args();
    if args.len() > 3 {
        fail("too many arguments");
    }

    let config = match rclip::config::ConfigContext::load() {
        Ok(config) => config,
        Err(rclip::config::Error::URLNotFound) => fail(format!("{}{}", rclip::URL_PARAM, " variable is missing from the config file\nPlease look at: https://github.com/noboruma/rclip-backends to setup a backend").as_str()),
        Err(rclip::config::Error::FileNotFound(s)) => fail(s.as_str()),
        Err(rclip::config::Error::DirNotFound(s)) => fail((s+"not found").as_str()),
        Err(rclip::config::Error::URLIllFormed) => fail(format!("{}{}", rclip::URL_PARAM, " variable has ill-formed URL format").as_str()),
    };

    let mut clipboard = rclip::Clipboard::from(config);

    match args.nth(1) {
        None => fail("not enough arguments"),
        Some(arg) => match arg.as_ref() {
            "open" => {
                match args.next() {
                    Some(_) => fail("too many arguments"),
                    None => {
                        fail_error(&clipboard.open());
                        let config = &clipboard.config;
                        config.store();
                        let _ = stdout.lock().write(&config.token.as_bytes()[..HASH_SHORT_SIZE]);
                    }
                }
            },
            "link" => {
                match args.next() {
                    Some(text) => {
                        fail_error(&clipboard.link(&mut text.as_bytes()));
                        clipboard.config.store();
                        let _ = stdout.lock().write("You are all set!\nYou can start using push/pull\n".as_ref());
                    }
                    None => fail("not enough arguments"),
                }
            },
            "push" => {
                let mut input : Box<dyn Read> = match args.next() {
                    Some(text) => Box::new(rclip::stream::StringStream::from(text)),
                    None => Box::new(stdin.lock()),
                };
                fail_error(&clipboard.push(&mut input));
            },
            "pull" => {
                let _ = match args.next() {
                    Some(_) => fail("too many arguments"),
                    None => fail_error(&clipboard.pull(&mut stdout.lock())),
                };
            },
            "-h" | "--help" | _ => stdout.lock().write_all(HELP).unwrap(),
        },
    }
}
