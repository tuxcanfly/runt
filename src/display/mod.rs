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

use std::io::{Stdout};

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
            if data.name.prefix == None {
                match &*data.name.local {
                    "img" => {
                        let attrs = data.attributes.borrow();
                        let alt = attrs.get("alt");
                        if let Some(alt) = alt {
                            print!(r#"<img alt="{}">"#, alt);
                        } else {
                            print!("<img>");
                        }
                        return;
                    }
                    "script" | "head" | "style" => {
                        return;
                    }
                    "q" => {
                        print!("\"");
                    }
                    _ => {}
                }
            }
            {
                let mut node = node.first_child();
                while let Some(child) = node {
                    display(terminal, &child, depth + 1, area);
                    node = child.next_sibling();
                }
            }
            if data.name.prefix == None {
                match &*data.name.local {
                    "a" => {
                        let attrs = data.attributes.borrow();
                        let href = attrs.get("href");
                        if let Some(href) = href {
                            print!(r#"<{}>"#, href);
                        } else {
                            print!("<>");
                        }
                    }
                    "q" => {
                        print!("\"");
                    }
                    _ => {}
                }
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
}
