# ankou
`/ɑːnkuː/`  
An OSINT git miner focused on enumerating security issues in JavaScript engines.


[![crates.io-badge]][crates.io] [![docs.rs-badge]][docs.rs]  
[![build-badge]][build]  
[![license]][lic]


_Current only supports WebKit (which is also still a WIP)_

[crates.io]: https://crates.io/crates/ankou
[crates.io-badge]:  https://img.shields.io/crates/v/ankou.svg

[docs.rs]: https://docs.rs/ankour
[docs.rs-badge]: https://docs.rs/ankou/badge.svg

[build]: https://github.com/drtychai/ankou/actions?query=workflow:build
[build-badge]: https://github.com/drtychai/ankou/workflows/build/badge.svg

[license]: https://img.shields.io/crates/l/ankou.svg
[lic]: /LICENSE

## Setup
`ak` requires the target repository's list of commit objects. The easiest way
to grab it is to add the `-n/--no-checkout` option when running `git clone`:

```bash
# Direct clone
git clone -j`nproc` --no-checkout git://git.webkit.org/WebKit.git
cargo run --bin ak webkit -l path/to/cloned/WebKit

```

## Usage

```bash
ankou 0.0.2
security-focused OSINT git miner

USAGE:
    ak [FLAGS] [OPTIONS] [api-key] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Verbose

OPTIONS:
    -d, --depth <depth>    Maximum depth of parent references to walk [default: 10]

ARGS:
    <api-key>    Bugzilla API key [env: AK_KEY]

SUBCOMMANDS:
    chromium    Enumerate the Google Chromium engine
    gecko       Enumerate the Mozilla Gecko engine
    help        Prints this message or the help of the given subcommand(s)
    webkit      Enumerate the Apple WebKit engine
```

