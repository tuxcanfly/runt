use kuchiki::{Node, NodeData};
use tui::layout::{Rect};
use tui::widgets::Paragraph;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::layout::Layout;
use tui::layout::Direction;
use tui::layout::Constraint;

use tui::backend::TermionBackend;
use tui::Terminal;

use std::io::Stdout;
use std::io::BufRead;

#[derive(Debug)]
struct Widget {
    content: String,
    area: Rect,
}

pub fn display(terminal: &mut Terminal<TermionBackend<Stdout>>, node: &Node, depth: u32, area: &mut Rect) {
    // style.show();
    let mut widgets: Vec<Widget> = vec![];
    match node.data() {
        NodeData::Text(contents) => {
            let contents = &**contents.borrow();
            let contents = contents.split_whitespace().collect::<Vec<_>>().join(" ");

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Max(1),
                        Constraint::Min(1),
                    ]
                        .as_ref(),
                )
                .split(*area);

            *area = chunks[2];

            let widget = Widget {
                area: chunks[0],
                content: contents,
            };
            widgets.push(widget);
        }
        NodeData::Element(ref data) => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(terminal, &child, depth + 1, area);
                node = child.next_sibling();
            }
        }
        _ => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(terminal, &child, depth + 1, area);
                node = child.next_sibling();
            }
        }
    }
    terminal.draw(|f| {
        // Render this widget's content
        for widget in widgets {
            let paragraph = Paragraph::new(widget.content).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, widget.area);
        }
    }).unwrap();

    // Wait for user input before closing the application
    let stdin = std::io::stdin();
    let _event = stdin.lock().read_line(&mut String::new());
}
