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

pub fn split(area: Rect, dir: tui::layout::Direction, ratio: (u16, u16)) -> Vec<Rect> {
    Layout::default()
        .direction(dir)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(ratio.0),
                Constraint::Percentage(ratio.1),
            ]
                .as_ref(),
        )
        .split(area)
}

pub fn display(terminal: &mut Terminal<TermionBackend<Stdout>>, node: &Node, depth: u32, area: &mut Rect, debug: bool) {
    // style.show();
    let mut widgets: Vec<Widget> = vec![];
    match node.data() {
        NodeData::Text(contents) => {
            let contents = &**contents.borrow();
            let contents = contents.split_whitespace().collect::<Vec<_>>().join(" ");

            if debug {
                println!("{}", contents);
            }
            let rects = split(*area, Direction::Horizontal, (50, 50));
            let top = rects[0];
            *area = rects[1];
            let widget = Widget {
                area: top, // TODO fix overwrite
                content: contents,
            };
            widgets.push(widget);
        }
        NodeData::Element(ref data) => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(terminal, &child, depth + 1, area, debug);
                node = child.next_sibling();
            }
        }
        _ => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(terminal, &child, depth + 1, area, debug);
                node = child.next_sibling();
            }
        }
    }
    if !debug {
        terminal.draw(|f| {
            for widget in widgets {
                let paragraph = Paragraph::new(widget.content).block(Block::default().borders(Borders::ALL));
                f.render_widget(paragraph, widget.area);
            }
        }).unwrap();

        // Wait for user input before closing the application
        let stdin = std::io::stdin();
        let _event = stdin.lock().read_line(&mut String::new());
    }
}
