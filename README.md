# rclip

[![Build Status](https://travis-ci.org/noboruma/rclip.svg?branch=master)](https://travis-ci.org/noboruma/rclip)
[![codecov](https://codecov.io/gh/noboruma/rclip/branch/master/graph/badge.svg)](https://codecov.io/gh/noboruma/rclip)
[![crates.io](https://img.shields.io/crates/v/remote-clipboard.svg)](https://crates.io/crates/remote-clipboard)

rclip is a remote clipboard. It allows users to push and pull data remotely from two different machines.

Linux and OSX are supported. Windows is not actively tested but should work as well.

## Table of Contents
* [Usage](#usage-demo)
    * [Back-ends](#back-ends)
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
    push [data]
        Copy the data to the remote clipboard
    pull
        Copy the data from the remote clipboard

```

### Usage demo

<a href="https://asciinema.org/a/340483?loop=1&autoplay=1&speed=2"><img src="https://asciinema.org/a/340483.svg" width="400"/></a>

### Back-ends

The CLI requires a remote back-end to communicate with.
You need to provide a URL in the `$HOME/.rclip.env` file such as:
```
URL=https://blah.amazonaws.com/dev
```

You can choose and deploy a back-end for your own usage:
https://github.com/noboruma/rclip-backends

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
