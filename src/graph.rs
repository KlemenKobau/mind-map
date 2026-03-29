use ollama_rs::generation::parameters::JsonSchema;
use petgraph::graph::{Node, UnGraph};
use serde::Deserialize;

#[derive(JsonSchema, Deserialize, Debug)]
pub struct NodeData {
    pub name: String,
}

#[derive(Debug)]
pub struct EdgeData {}

#[derive(Debug)]
pub struct GraphManager {
    graph: UnGraph<NodeData, EdgeData>,
}

impl GraphManager {
    pub fn add_node(&mut self, name: String) {
        let node_data = NodeData { name };

        self.graph.add_node(node_data);
    }

    pub fn get_all_nodes(&self) -> &[Node<NodeData>] {
        self.graph.raw_nodes()
    }
}
