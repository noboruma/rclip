# rclip

[![Build Status](https://travis-ci.org/noboruma/rclip.svg?branch=master)](https://travis-ci.org/noboruma/rclip)
[![codecov](https://codecov.io/gh/noboruma/rclip/branch/master/graph/badge.svg)](https://codecov.io/gh/noboruma/rclip)
[![crates.io](https://img.shields.io/crates/v/remote-clipboard.svg)](https://crates.io/crates/remote-clipboard)

rclip is a remote clipboard. It allows users to copy and paste data remotely from two different machines.

Linux and OSX are supported. Windows is not actively tested but should work as well.

## Table of Contents
* [Usage](#usage-demo)
    * [Backend](#backend)
* [Build & Test](#build--test)
* [License](#license)

## man rclip:

```
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

```

### Usage demo
<a href="https://asciinema.org/a/342749?loop=1&autoplay=1&speed=2"><img src="https://asciinema.org/a/340483.svg" width="400"/></a>

### Backend

The CLI requires a remote backend to store & serve a remote clipboard.

To simplify the tool usage, a default backend is provided by the tool maintainer.
Be aware that the default backend does not make any guarantee regarding the data your provide: it can be accessed publicly. Although the design tries to make it hard, clipboard can theoretically end up being shared (hash collision). Consider encrypting your data, or use your own backend if your data is sensitive.
Use the default backend at your own risk.

To mitigate the default backend weaknesses, it is highly recommended that you choose & deploy your own backend, please see how to:
https://github.com/noboruma/rclip-backends

To use your own backend, please provide a URL in the `$HOME/.rclip.env` file such as:
```
RCLIP_URL=https://blah.amazonaws.com/
```
#### Setup demo

<a href="https://asciinema.org/a/340551?loop=1&autoplay=1&speed=3"><img src="https://asciinema.org/a/340551.svg" width="400"/></a>

/!\ The proposed solution is far from being optimal. This is work in progress.

## Build & Test

Build with:
```
cargo build
```

To run unit tests:
```
cargo test
```
## License

Published under the MPL 2.0 license.
