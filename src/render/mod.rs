use tui::backend::TermionBackend;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;


use std::io::BufRead;

fn render() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal backend
    let stdout = std::io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a widget
    let widget = Paragraph::new(Spans::from(vec![
        Span::raw("Hello "),
        Span::styled("World!", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL));

    // Set up the terminal and render the widget
    terminal.clear()?;
    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(widget, size);
    })?;

    // Wait for user input before closing the application
    let stdin = std::io::stdin();
    let _event = stdin.lock().read_line(&mut String::new())?;

    Ok(())
}

