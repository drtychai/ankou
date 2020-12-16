# ankou
`/ɑːnkuː/`  
An OSINT git miner focused on enumerating security issues in a [browser] repo.

_Current only supports WebKit (which is also still a WIP)_

## Usage
```bash
ankou 0.0.1
security-focused OSINT git miner

USAGE:
    ak [FLAGS] [OPTIONS] [ARGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Verbose

OPTIONS:
    -d, --depth <depth>    Maximum depth of parent references to walk

ARGS:
    <api-key>    Bugzilla API key [env: AK_KEY]
    <bug-id>     Specific Bug ID to lookup [env: AK_BUG_ID]

SUBCOMMANDS:
    chromium    Enumerate the Google Chromium engine
    gecko       Enumerate the Mozilla Gecko engine
    help        Prints this message or the help of the given subcommand(s)
    webkit      Enumerate the Apple WebKit engine
```

