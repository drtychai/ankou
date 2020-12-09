extern crate simple_logger;
extern crate git2;
extern crate reqwest;

#[macro_use]
mod type_info;
mod logger;
mod git;

use ::std::{io,env};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::get_logger().init().unwrap();

    // Bugzilla API testing block
    {
        // Retrieve Bugzilla API key and target BugID from runtime environment
        // variables `AK_KEY` and `AK_BUG_ID`, respectively, where:
        //   api_key -> defaults to an empty string
        //   bud_id  -> required else panic! bailout 
        let (api_key, bug_id) : (String, String) = match (env::var("AK_KEY"), env::var("AK_BUG_ID")) {
            (Ok(k), Ok(l)) => (k, l),
            (_, Ok(l)) => ("".to_owned(), l),
            _ => panic!("Error: environment variable 'AK_BUG_ID' must not be empty"),
        };
    
        let repo_uri: String = format!("https://bugs.webkit.org/rest/bug?api_key={}&id={}", api_key, bug_id.clone());
    
        log::info!("Requesting bug ID {} for WebKit...", bug_id);
        let mut resp = reqwest::blocking::get(&repo_uri)?;
    
        log::debug!("Response: ");
        resp.copy_to(&mut io::stdout())?;
    }

    // git2-rs api testing block
    {
        git::log(vec!["".to_owned()],0,50).unwrap();
    }


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::git::Repository;
    use super::type_info::*;
    use ::std::env;
    use ::std::fs::{create_dir, remove_dir_all};
    use ::std::path::PathBuf;

    #[test]
    fn clone() {
        let repo = "https://github.com/drtychai/dotfiles";
        let test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("akmn");

        create_dir(test_dir.clone()).unwrap();

        // Clone into our test directory
        assert_eq!(
            type_of!(Repository::clone(repo, test_dir.clone().join("clone_test")).unwrap()),
            type_of!(Repository::init_bare(test_dir.clone().join("clone_test_init")).unwrap())
        );

        remove_dir_all(test_dir.clone()).unwrap();
    }
}
