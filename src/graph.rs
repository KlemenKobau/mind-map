use ollama_rs::generation::parameters::JsonSchema;
use serde::Deserialize;

#[derive(JsonSchema, Deserialize, Debug, Clone)]
pub struct NodeData {
    pub name: String,
    pub description: String,
    pub related: Vec<String>,
}
