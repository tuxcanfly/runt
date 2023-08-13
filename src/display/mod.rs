use kuchiki::{Node, NodeData};
use tui::layout::{Rect};
use tui::layout::Layout;
use tui::layout::Direction;
use tui::layout::Constraint;

#[derive(Debug)]
pub struct Widget {
    pub content: String,
    pub area: Rect,
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

pub fn display(node: &Node, depth: u32, widgets: &mut Vec<Widget>, area: &mut Rect) {
    match node.data() {
        NodeData::Text(contents) => {
            let contents = &**contents.borrow();
            let contents = contents.split_whitespace().collect::<Vec<_>>().join(" ");
            if contents == "" {
                return
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
                display(&child, depth + 1, widgets, area);
                node = child.next_sibling();
            }
        }
        _ => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(&child, depth + 1, widgets, area);
                node = child.next_sibling();
            }
        }
    }
}
