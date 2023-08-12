extern crate clap;

use clap::{App, Arg};

use url::Url;

use tui::Terminal;
use tui::backend::TermionBackend;

mod display;
mod fetcher;
mod page;

fn prepend_https(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("file://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("Runt terminal-based web browser")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug mode")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("The URL to parse")
                .takes_value(true),
            )
            .get_matches();

    // Initialize the terminal backend
    let stdout = std::io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    let  area = &mut terminal.size().unwrap();
    let debug = matches.is_present("debug");

    if let Some(url) = matches.value_of("url") {
        match Url::parse(&prepend_https(url)) {
            Ok(parsed_url) => {
                let page = page::fetch(parsed_url).await?;
                display::display(&mut terminal, &page.document, 0, area, debug);
                println!("");
            }
            Err(err) => {
                println!("failed to parse urL: {}", err);
            }
        }
    }
    Ok(())
}
