extern crate clap;

use clap::{App, Arg};

use url::Url;

use tui::Terminal;
use tui::backend::TermionBackend;

use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;

use std::io::BufRead;

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

    if let Some(url) = matches.value_of("url") {
        match Url::parse(&prepend_https(url)) {
            Ok(parsed_url) => {
                let page = page::fetch(parsed_url).await?;
                let mut widgets = vec![];
                display::display(&page.document, 0, &mut widgets, area);
                terminal.draw(|f| {
                    for widget in widgets {
                        let paragraph = Paragraph::new(widget.content).block(Block::default().borders(Borders::ALL));
                        f.render_widget(paragraph, widget.area);
                    }
                }).unwrap();
            }
            Err(err) => {
                println!("failed to parse urL: {}", err);
            }
        }
    }
    // Wait for user input before closing the application
    let stdin = std::io::stdin();
    let _event = stdin.lock().read_line(&mut String::new())?;
    Ok(())
}
