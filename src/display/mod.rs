use kuchiki::{Node, NodeData};

#[derive(Debug)]
pub struct Widget {
    pub content: String,
}

pub fn display(node: &Node, depth: u32, widgets: &mut Vec<Widget>) {
    match node.data() {
        NodeData::Text(contents) => {
            let contents = &**contents.borrow();
            let contents = contents.split_whitespace().collect::<Vec<_>>().join(" ");
            if contents.len() < 20 {
                return
            }
            let widget = Widget {
                content: contents,
            };
            widgets.push(widget);
        }
        NodeData::Element(ref data) => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(&child, depth + 1, widgets);
                node = child.next_sibling();
            }
        }
        _ => {
            let mut node = node.first_child();
            while let Some(child) = node {
                display(&child, depth + 1, widgets);
                node = child.next_sibling();
            }
        }
    }
}
