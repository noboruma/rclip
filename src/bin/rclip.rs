use remote_clipboard as rclip;
use std::env;
use std::io::{self, Write};

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

fn main() {

    let stdout = io::stdout();

    let mut args = env::args();
    if args.len() > 3 {
        rclip::fail("too many arguments");
    }

    match args.nth(1) {
        None => rclip::fail("not enough arguments"),
        Some(arg) => match arg.as_ref() {
            "open" => {
                match args.next() {
                    Some(_) => {
                        rclip::fail("too many arguments");
                    },
                    None => {
                        // DO
                        rclip::open(stdout);
                    },
                }
            },
            "link" => {
                match args.next() {
                    Some(text) => {
                        rclip::link(stdout, &mut text.as_bytes());
                    },
                    None => {
                        rclip::fail("not enough arguments");
                    },
                }
            },
            "push" => {
                match args.next() {
                    Some(text) => {
                        rclip::push(&mut text.as_bytes())
                    },
                    None => {
                        let stdin = io::stdin();
                        rclip::push(&mut stdin.lock());
                    },
                }
            },
            "pull" => {
                match args.next() {
                    Some(_) => {
                        rclip::fail("too many arguments");
                    },
                    None => {
                        rclip::pull(stdout);
                    },
                }
            },
            "-h" | "--help" | _ => {
                stdout.lock().write_all(HELP).unwrap();
            },
        },
    }
}
