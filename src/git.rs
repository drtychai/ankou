pub use git2::{Commit, DiffOptions, Error, ObjectType, Repository, Signature, Time};

pub fn log(path: ::std::path::PathBuf, depth: usize) -> Result<Vec<String>, Error> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::NONE)?;
    revwalk.push_head()?;

    // Filter our revwalk based on the CLI parameters
    macro_rules! filter_try {
        ($e:expr) => {
            match $e {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            }
        };
    }

    let revwalk = revwalk.filter_map(|id| {
        let id = filter_try!(id);
        let commit = filter_try!(repo.find_commit(id));
        Some(Ok(commit))
    }).take(depth);

  
    let mut bug_uris: Vec<String> = Vec::new();
    for commit in revwalk {
        let commit = commit?;
        match find_bug_id(&commit) {
            Some(s) => bug_uris.push(s),
            None => continue,
        }
    }

    log::info!("Bug URIs: {:?}", bug_uris);

    Ok(bug_uris)
}

/// Returns the six digit Bug ID found in the commit message
/// --> matches pattern "https://bugs.webkit.org/show_bug.cgi?id=[0-9]{6}"
fn find_bug_id(commit: &Commit) -> Option<String> {
    let pattern = "bug.cgi?id=";
    let lower_bound = pattern.len();
    let commit_body = String::from_utf8_lossy(commit.message_bytes());

    // Extracts only the 6 digit BugID
    let bug_uri = match commit_body.clone().find(pattern) {
        Some(i) => Some(
            commit_body[i+lower_bound..i+lower_bound+6].to_owned()
        ),
        None => None,
    };

    bug_uri
}
