# ankou
`/ɑːnkuː/`  
An OSINT git miner focused on enumerating security issues in a [browser] repo.

_Current only supports WebKit (which is also still a WIP)_

## Usage
```bash
ankou 0.1.0

USAGE:
    ankou [FLAGS] [OPTIONS] --target-engine [<webkit>,<gecko>,<chromium>]

FLAGS:
    -h, --help                   Prints help information
    -t, --target-engine <trgt>   Valid target names: webkit, gecko, chromium
    -V, --version                Prints version information

OPTIONS:
    -r, --repository <repo>      The remote URI where the target repository is housed 
        --from-date <from_date>  Date that must preceed the oldest commit
        --to-date <to_date>      Date that must proceed the newest commit
```

