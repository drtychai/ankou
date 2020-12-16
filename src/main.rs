extern crate structopt;
extern crate simple_logger;
extern crate git2;
extern crate reqwest;

#[macro_use]
mod type_info;
mod git;

use crate::simple_logger::SimpleLogger;
use crate::structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "ankou",
    about = "security-focused OSINT git miner",
)]
struct Args {
    /// Target engine (WebKit, V8, Gecko)
    #[structopt(subcommand)]
    cmd: Command,

    /// Maximum depth of parent references to walk
    #[structopt(short, long, default_value = "10")]
    depth: usize,

    /// Bugzilla API key
    #[structopt(env = "AK_KEY", hide_env_values = true, required_if("cmd", "webkit"))]
    api_key: Option<String>,

    ///// Specific Bug ID to lookup
    //#[structopt(env = "AK_BUG_ID", hide_env_values = true)]
    //bug_id: Option<String>,

    /// Verbose
    #[structopt(short, global=true)]
    verbose: bool,
}

#[derive(Debug, StructOpt, Clone)]
enum Command {
    /// Enumerate the Apple WebKit engine
    #[structopt(name = "webkit")]
    Webkit {
        #[structopt(short, parse(from_os_str))]
        local_path: ::std::path::PathBuf,
        
        #[structopt(skip = "https://bugs.webkit.org/rest/bug")]
        bugzilla_api_endpoint: String,
    },

    /// Enumerate the Google Chromium engine
    #[structopt(name = "chromium")]
    Chromium {
        #[structopt(short, parse(from_os_str))]
        local_path: ::std::path::PathBuf,
        
        #[structopt(skip = "")]
        bugzilla_api_endpoint: String,
    },

    #[structopt(name = "gecko")]
    /// Enumerate the Mozilla Gecko engine
    Gecko {
        #[structopt(short, parse(from_os_str))]
        local_path: ::std::path::PathBuf,
        
        #[structopt(skip = "")]
        bugzilla_api_endpoint: String,
    },
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize our logger
    let level = match args.verbose {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };
    SimpleLogger::new().with_level(level).init().unwrap();

    // Prep CLI args into local vars
    let api_key = match args.clone().api_key {
        Some(k) => k,
        None => panic!("An API key is required to query external bug details. Please set the AK_KEY environment variable"),
    };

    let (path, base_uri): (::std::path::PathBuf, String) = match args.cmd {
        Command::Webkit { local_path : path, bugzilla_api_endpoint: uri} => (path, uri),
        Command::Chromium { local_path : path, bugzilla_api_endpoint: uri} => (path, uri),
        Command::Gecko { local_path : path, bugzilla_api_endpoint: uri} => (path, uri),
    };

    // Collect all bug IDs from repository commits
    let bugs: Vec<String> = match git::log(path.clone(), args.depth) {
            Ok(s) => s,
            Err(e) => panic!("Issues collecting bug IDs. Err: {:?}", e),
    };
   
    // Request and sort bug details
    for bug in bugs {
        let repo_uri: String = format!("{}?api_key={}&id={}", base_uri, api_key, bug.clone());

        log::debug!("Requesting bug ID {} for WebKit...", repo_uri.clone());
        let mut resp = reqwest::blocking::get(&repo_uri)?;

        log::debug!("Response: ");
        resp.copy_to(&mut ::std::io::stdout())?;
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::git;
    use super::type_info::*;
    use ::std::env;
    use ::std::fs::{create_dir, remove_dir_all};
    use ::std::path::PathBuf;

    #[test]
    fn git_clone_types() {
        let repo = "https://github.com/drtychai/dotfiles";
        let test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("akmn");

        create_dir(test_dir.clone()).unwrap();

        // Clone into our test directory
        assert_eq!(
            type_of!(git::Repository::clone(repo, test_dir.clone().join("clone_test")).unwrap()),
            type_of!(git::Repository::init_bare(test_dir.clone().join("clone_test_init")).unwrap())
        );

        remove_dir_all(test_dir.clone()).unwrap();
    }
}
