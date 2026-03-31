use dioxus::prelude::*;
use petgraph::stable_graph::{NodeIndex, StableUnGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};

use crate::layout::place_children;
use crate::ollama::expand_topic;

mod graph;
mod layout;
mod ollama;

#[derive(Clone, PartialEq)]
struct UiNode {
    name: String,
    description: String,
    x: f64,
    y: f64,
    expanded: bool,
    parent_angle: f64,
}

fn node_fill(expanded: bool, selected: bool) -> &'static str {
    if selected { "#7b2d8b" } else if expanded { "#533483" } else { "#1a5276" }
}

fn node_stroke(selected: bool) -> &'static str {
    if selected { "#e040fb" } else { "#8888cc" }
}

fn build_initial_graph(data: crate::graph::NodeData) -> StableUnGraph<UiNode, ()> {
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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut graph = use_signal(StableUnGraph::<UiNode, ()>::default);
    let mut topic_input = use_signal(String::new);
    let mut status = use_signal(|| "Enter a topic".to_string());
    let mut selected_id = use_signal(|| Option::<NodeIndex>::None);
    let mut loading = use_signal(|| false);

    let mut on_submit = move |_| {
        let topic = topic_input.read().trim().to_string();
        if topic.is_empty() || *loading.read() {
            return;
        }
        loading.set(true);
        graph.set(StableUnGraph::default());
        selected_id.set(None);
        status.set("Generating...".to_string());

        spawn(async move {
            match expand_topic(&topic).await {
                Ok(data) => {
                    graph.set(build_initial_graph(data));
                    status.set("Click a node to expand".to_string());
                }
                Err(e) => status.set(format!("Error: {e}")),
            }
            loading.set(false);
        });
    };

    let mut on_node_click = move |idx: NodeIndex| {
        if *loading.read() {
            return;
        }
        let node = graph.read().node_weight(idx).cloned();
        let Some(node) = node else { return };

        if node.expanded {
            selected_id.set(Some(idx));
            return;
        }

        let (topic, px, py, pa) = (node.name.clone(), node.x, node.y, node.parent_angle);
        loading.set(true);
        selected_id.set(Some(idx));
        status.set("Generating...".to_string());

        spawn(async move {
            match expand_topic(&topic).await {
                Ok(data) => {
                    let existing: Vec<String> =
                        graph.read().node_weights().map(|n| n.name.clone()).collect();
                    let new_related: Vec<String> = data
                        .related
                        .into_iter()
                        .filter(|r| !existing.contains(r))
                        .collect();
                    let positions = place_children(px, py, pa, new_related.len());

                    {
                        let mut g = graph.write();
                        for (name, (cx, cy, angle)) in new_related.iter().zip(&positions) {
                            let child = g.add_node(UiNode {
                                name: name.clone(),
                                description: String::new(),
                                x: *cx,
                                y: *cy,
                                expanded: false,
                                parent_angle: *angle,
                            });
                            g.add_edge(idx, child, ());
                        }
                        g[idx].expanded = true;
                        g[idx].description = data.description;
                    }

                    status.set("Click a node to expand".to_string());
                }
                Err(e) => status.set(format!("Error: {e}")),
            }
            loading.set(false);
        });
    };

    let graph_snap = graph.read().clone();
    let selected = *selected_id.read();
    let selected_node = selected.and_then(|idx| graph_snap.node_weight(idx)).cloned();

    rsx! {
        div {
            style: "display:flex; flex-direction:column; height:100vh; font-family:sans-serif; background:#0d0d1a; color:#ccc;",

            div {
                style: "display:flex; gap:8px; padding:10px; background:#111122; border-bottom:1px solid #333;",
                input {
                    style: "flex:1; padding:6px 10px; background:#1a1a2e; color:#eee; border:1px solid #555; border-radius:4px; font-size:14px;",
                    placeholder: "Enter a topic...",
                    value: "{topic_input}",
                    oninput: move |e| topic_input.set(e.value()),
                    onkeydown: move |e| if e.key() == Key::Enter { on_submit(()) },
                }
                button {
                    style: "padding:6px 16px; background:#1a5276; color:#eee; border:none; border-radius:4px; cursor:pointer; font-size:14px;",
                    disabled: *loading.read(),
                    onclick: move |_| on_submit(()),
                    if *loading.read() { "Loading..." } else { "Generate" }
                }
            }

            div {
                style: "display:flex; flex:1; overflow:hidden;",

                svg {
                    style: "flex:1;",
                    "viewBox": "0 0 900 700",
                    preserve_aspect_ratio: "xMidYMid meet",

                    for edge_ref in graph_snap.edge_references() {
                        {
                            let f = &graph_snap[edge_ref.source()];
                            let t = &graph_snap[edge_ref.target()];
                            rsx! {
                                line {
                                    x1: "{f.x}", y1: "{f.y}",
                                    x2: "{t.x}", y2: "{t.y}",
                                    stroke: "#8888cc",
                                    "strokeWidth": "2",
                                }
                            }
                        }
                    }

                    for node_idx in graph_snap.node_indices() {
                        {
                            let node = &graph_snap[node_idx];
                            let is_selected = Some(node_idx) == selected;
                            let (cx, cy, name) = (node.x, node.y, node.name.clone());
                            rsx! {
                                g {
                                    onclick: move |_| on_node_click(node_idx),
                                    style: "cursor:pointer;",
                                    circle {
                                        cx: "{cx}", cy: "{cy}", r: "40",
                                        fill: node_fill(node.expanded, is_selected),
                                        stroke: node_stroke(is_selected),
                                        "strokeWidth": "2",
                                    }
                                    text {
                                        x: "{cx}", y: "{cy}",
                                        fill: "white",
                                        "fontSize": "11",
                                        "textAnchor": "middle",
                                        "dominantBaseline": "middle",
                                        "{name}"
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    style: "width:240px; padding:16px; background:#111122; border-left:1px solid #333; overflow-y:auto;",
                    if let Some(node) = selected_node {
                        h3 { style: "margin:0 0 8px; color:#e040fb; font-size:15px;", "{node.name}" }
                        p { style: "margin:0; font-size:13px; line-height:1.5; color:#bbb;", "{node.description}" }
                    } else {
                        p { style: "color:#555; font-size:13px;", "Select a node to see its description." }
                    }
                }
            }

            div {
                style: "padding:6px 12px; background:#111122; border-top:1px solid #333; font-size:12px; color:#888;",
                "{status}"
            }
        }
    }
}
