extern crate simple_logger;
extern crate log;
extern crate git2;

use log::info;
use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

mod git;
mod utils;
// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    utils::get_logger().init().unwrap();

    // Some simple CLI args requirements...
    let url = match ::std::env::args().nth(1) {
        Some(url) => url,
        None => {
            info!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        info!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    info!("Response: {}", res.status());
    info!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    info!("\n\nDone!");

    Ok(())
}
