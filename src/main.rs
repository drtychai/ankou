#[allow(unused_imports,dead_code)]
extern crate simple_logger;
extern crate log;
extern crate git2;

//use hyper::{body::HttpBody as _, Client};
//use tokio::io::{self, AsyncWriteExt as _};

mod git;

mod utils;
//use utils::Result;

fn main() {
    match utils::get_logger().init() {
        Ok(()) => {},
        Err(e) => panic!("Logger initalization failed, err: {}", e),
    }

    git::log(vec![String::from("")], 1, 0).unwrap();


}


//#[tokio::main]
//async fn main() -> Result<()> {
//    utils::get_logger().init().unwrap();
//
//    // Some simple CLI args requirements...
//    let url = match ::std::env::args().nth(1) {
//        Some(url) => url,
//        None => {
//            log::info!("Usage: client <url>");
//            return Ok(());
//        }
//    };
//
//    // HTTPS requires picking a TLS implementation, so give a better
//    // warning if the user tries to request an 'https' URL.
//    let url = url.parse::<hyper::Uri>().unwrap();
//    if url.scheme_str() != Some("http") {
//        log::info!("This example only works with 'http' URLs.");
//        return Ok(());
//    }
//
//    fetch_url(url).await
//}
//
//async fn fetch_url(url: hyper::Uri) -> Result<()> {
//    let client = Client::new();
//
//    let mut res = client.get(url).await?;
//
//    log::info!("Response: {}", res.status());
//    log::info!("Headers: {:#?}\n", res.headers());
//
//    // Stream the body, writing each chunk to stdout as we get it
//    // (instead of buffering and printing at the end).
//    while let Some(next) = res.data().await {
//        let chunk = next?;
//        io::stdout().write_all(&chunk).await?;
//    }
//
//    log::info!("\n\nDone!");
//
//    Ok(())
//}
