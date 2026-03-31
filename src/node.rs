#[derive(Clone, PartialEq)]
pub struct UiNode {
    pub name: String,
    pub description: String,
    pub expanded: bool,
}

pub fn node_fill(expanded: bool, selected: bool) -> &'static str {
    if selected {
        "#7b2d8b"
    } else if expanded {
        "#533483"
    } else {
        "#1a5276"
    }
}

pub fn node_stroke(selected: bool) -> &'static str {
    if selected { "#e040fb" } else { "#8888cc" }
}
