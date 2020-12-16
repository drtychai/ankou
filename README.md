# ankou
`/ɑːnkuː/`  
An OSINT git miner focused on enumerating security issues in a [browser] repo.

_Current only supports WebKit (which is also still a WIP)_

## Usage
```bash
ankou 0.0.1
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

