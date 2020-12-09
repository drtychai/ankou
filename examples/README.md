Example pull from [git2-rs](https://github.com/rust-lang/git2-rs/)

```
ankou 0.1.0

USAGE:
    cargo run --example git-log -- [FLAGS] [OPTIONS] [max-parents] [min-parents] [commit]... [-- <spec>...]

FLAGS:
        --date-order        sort commits in date order
    -h, --help              Prints help information
        --merges            only show merge commits
        --no-max-parents    don't require a maximum number of parents
        --no-merges         don't show merge commits
        --no-min-parents    don't require a minimum number of parents
    -p, --patch             show commit diff
        --reverse           sort commits in reverse
        --topo-order        sort commits in topological order
    -V, --version           Prints version information

OPTIONS:
        --author <author>          author to sort by
        --committer <committer>    committer to sort by
        --git-dir <dir>            alternative git directory to use
    -n, --max-count <max-count>    maximum number of commits to show
        --grep <pat>               pattern to filter commit messages by
        --skip <skip>              number of commits to skip

ARGS:
    <max-parents>    specify a maximum number of parents for a commit
    <min-parents>    specify a minimum number of parents for a commit
    <commit>...      
    <spec>...       
``` 
