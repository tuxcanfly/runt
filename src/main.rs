extern crate clap;
use std::default::Default;

use clap::{App, Arg};

use url::Url;

mod display;
mod fetcher;
mod page;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("Runt terminal-based web browser")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("The URL to parse")
                .takes_value(true),
            )
            .get_matches();

    if let Some(url) = matches.value_of("url") {
        match Url::parse(url) {
            Ok(parsed_url) => {
                println!("Parsed URL: {:?}", parsed_url);
                let page = page::fetch(parsed_url).await?;
                display::display(&page.document, 0, Default::default());
                println!("");
            }
            Err(err) => {
                println!("failed to parse urL: {}", err);
            }
        }
    }
    Ok(())
}
