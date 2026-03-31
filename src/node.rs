use petgraph::stable_graph::StableUnGraph;

use crate::graph::NodeData;
use crate::layout::place_children;

#[derive(Clone, PartialEq)]
pub struct UiNode {
    pub name: String,
    pub description: String,
    pub x: f64,
    pub y: f64,
    pub expanded: bool,
    pub parent_angle: f64,
}

pub fn node_fill(expanded: bool, selected: bool) -> &'static str {
    if selected { "#7b2d8b" } else if expanded { "#533483" } else { "#1a5276" }
}

pub fn node_stroke(selected: bool) -> &'static str {
    if selected { "#e040fb" } else { "#8888cc" }
}

pub fn build_initial_graph(data: NodeData) -> StableUnGraph<UiNode, ()> {
    let mut g = StableUnGraph::default();
    let root = g.add_node(UiNode {
        name: data.name,
        description: data.description,
        x: 450.0,
        y: 350.0,
        expanded: true,
        parent_angle: 0.0,
    });
    let positions = place_children(450.0, 350.0, 0.0, data.related.len());
    for (name, (cx, cy, angle)) in data.related.into_iter().zip(positions) {
        let child = g.add_node(UiNode {
            name,
            description: String::new(),
            x: cx,
            y: cy,
            expanded: false,
            parent_angle: angle,
        });
        g.add_edge(root, child, ());
    }
    g
}
