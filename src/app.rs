use std::collections::HashSet;
use std::time::Duration;

use dioxus::prelude::*;
use fdg_sim::glam::Vec3;
use fdg_sim::petgraph::graph::NodeIndex;
use fdg_sim::petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

use crate::node::{UiNode, node_fill, node_stroke};
use crate::ollama::expand_topic;

const SCALE: f64 = 4.0;
const SVG_CX: f64 = 450.0;
const SVG_CY: f64 = 350.0;

fn to_svg(loc: Vec3) -> (f64, f64) {
    (loc.x as f64 * SCALE + SVG_CX, loc.y as f64 * SCALE + SVG_CY)
}

fn from_svg(sx: f64, sy: f64) -> Vec3 {
    Vec3::new(
        ((sx - SVG_CX) / SCALE) as f32,
        ((sy - SVG_CY) / SCALE) as f32,
        0.0,
    )
}

#[component]
pub fn App() -> Element {
    let mut sim = use_signal(|| {
        Simulation::<UiNode, ()>::from_graph(ForceGraph::default(), SimulationParameters::default())
    });
    let mut pinned: Signal<HashSet<NodeIndex>> = use_signal(HashSet::new);
    let mut topic_input = use_signal(String::new);
    let mut status = use_signal(|| "Enter a topic".to_string());
    let mut selected_id: Signal<Option<NodeIndex>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    let mut dragging: Signal<Option<NodeIndex>> = use_signal(|| None);
    let mut drag_offset = use_signal(|| (0.0_f64, 0.0_f64));
    let mut mouse_down_svg = use_signal(|| (0.0_f64, 0.0_f64));

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            tokio::time::sleep(Duration::from_millis(16)).await;
            {
                let mut s = sim.write();
                s.update(0.035);
                let drag_idx = *dragging.read();
                let to_freeze: Vec<NodeIndex> = {
                    let pinned_set = pinned.read();
                    let graph = s.get_graph();
                    graph
                        .node_indices()
                        .filter(|idx| drag_idx == Some(*idx) || pinned_set.contains(idx))
                        .collect()
                };
                let graph = s.get_graph_mut();
                for idx in to_freeze {
                    graph[idx].velocity = Vec3::ZERO;
                }
            }
        }
    });

    let mut on_submit = move |_| {
        let topic = topic_input.read().trim().to_string();
        if topic.is_empty() || *loading.read() {
            return;
        }
        loading.set(true);
        selected_id.set(None);
        pinned.write().clear();
        status.set("Generating...".to_string());

        spawn(async move {
            match expand_topic(&topic).await {
                Ok(data) => {
                    let mut graph: ForceGraph<UiNode, ()> = ForceGraph::default();
                    let root = graph.add_force_node(
                        data.name.clone(),
                        UiNode {
                            name: data.name,
                            description: data.description,
                            expanded: true,
                        },
                    );
                    for name in data.related {
                        let child = graph.add_force_node(
                            name.clone(),
                            UiNode {
                                name,
                                description: String::new(),
                                expanded: false,
                            },
                        );
                        graph.add_edge(root, child, ());
                    }
                    sim.write().set_graph(graph);
                    sim.write().reset_node_placement();
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
        let node_data = sim
            .read()
            .get_graph()
            .node_weight(idx)
            .map(|n| n.data.clone());
        let Some(node) = node_data else { return };

        if node.expanded {
            selected_id.set(Some(idx));
            return;
        }

        let topic = node.name.clone();
        loading.set(true);
        selected_id.set(Some(idx));
        status.set("Generating...".to_string());

        spawn(async move {
            match expand_topic(&topic).await {
                Ok(data) => {
                    let existing: Vec<String> = sim
                        .read()
                        .get_graph()
                        .node_weights()
                        .map(|n| n.data.name.clone())
                        .collect();
                    let new_related: Vec<String> = data
                        .related
                        .into_iter()
                        .filter(|r| !existing.contains(r))
                        .collect();

                    {
                        let mut s = sim.write();
                        let graph = s.get_graph_mut();
                        let parent_loc = graph[idx].location;
                        let count = new_related.len();
                        for (i, name) in new_related.iter().enumerate() {
                            let child = graph.add_force_node(
                                name.clone(),
                                UiNode {
                                    name: name.clone(),
                                    description: String::new(),
                                    expanded: false,
                                },
                            );
                            let angle = if count > 0 {
                                std::f32::consts::TAU * i as f32 / count as f32
                            } else {
                                0.0
                            };
                            graph[child].location =
                                parent_loc + Vec3::new(5.0 * angle.cos(), 5.0 * angle.sin(), 0.0);
                            graph.add_edge(idx, child, ());
                        }
                        graph[idx].data.expanded = true;
                        graph[idx].data.description = data.description;
                    }

                    status.set("Click a node to expand".to_string());
                }
                Err(e) => status.set(format!("Error: {e}")),
            }
            loading.set(false);
        });
    };

    let on_svg_mousemove = move |e: MouseEvent| {
        let Some(drag_idx) = *dragging.read() else {
            return;
        };
        let coords = e.element_coordinates();
        let (sx, sy) = (coords.x, coords.y);
        let (ox, oy) = *drag_offset.read();
        let new_loc = from_svg(sx - ox, sy - oy);
        let mut s = sim.write();
        let graph = s.get_graph_mut();
        graph[drag_idx].location = new_loc;
        graph[drag_idx].old_location = new_loc;
        graph[drag_idx].velocity = Vec3::ZERO;
    };

    let on_svg_mouseup = move |e: MouseEvent| {
        let Some(drag_idx) = *dragging.read() else {
            return;
        };
        let coords = e.element_coordinates();
        let (sx, sy) = (coords.x, coords.y);
        let (mdx, mdy) = *mouse_down_svg.read();
        let dist = ((sx - mdx).powi(2) + (sy - mdy).powi(2)).sqrt();
        if dist < 5.0 {
            on_node_click(drag_idx);
        } else {
            pinned.write().insert(drag_idx);
        }
        dragging.set(None);
    };

    let graph_snap = sim.read().get_graph().clone();
    let selected = *selected_id.read();
    let selected_node = selected
        .and_then(|idx| graph_snap.node_weight(idx))
        .map(|n| n.data.clone());

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
                    onmousemove: on_svg_mousemove,
                    onmouseup: on_svg_mouseup,

                    for edge_ref in graph_snap.edge_references() {
                        {
                            let f = &graph_snap[edge_ref.source()];
                            let t = &graph_snap[edge_ref.target()];
                            let (fx, fy) = to_svg(f.location);
                            let (tx, ty) = to_svg(t.location);
                            rsx! {
                                line {
                                    x1: "{fx}", y1: "{fy}",
                                    x2: "{tx}", y2: "{ty}",
                                    stroke: "#8888cc",
                                    "strokeWidth": "2",
                                }
                            }
                        }
                    }

                    for (node_idx, node) in graph_snap.node_references() {
                        {
                            let (cx, cy) = to_svg(node.location);
                            let is_selected = Some(node_idx) == selected;
                            let name = node.data.name.clone();
                            let expanded = node.data.expanded;
                            rsx! {
                                g {
                                    style: "cursor:pointer;",
                                    onmousedown: move |e| {
                                        let coords = e.element_coordinates();
                                        let (sx, sy) = (coords.x, coords.y);
                                        let (nx, ny) = to_svg(sim.read().get_graph()[node_idx].location);
                                        drag_offset.set((sx - nx, sy - ny));
                                        mouse_down_svg.set((sx, sy));
                                        dragging.set(Some(node_idx));
                                    },
                                    circle {
                                        cx: "{cx}", cy: "{cy}", r: "40",
                                        fill: node_fill(expanded, is_selected),
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
