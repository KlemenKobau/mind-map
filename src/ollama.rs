use anyhow::Result;
use ollama_rs::{
    Ollama,
    generation::completion::request::GenerationRequest,
    generation::parameters::{FormatType, JsonStructure},
};

use crate::graph::NodeData;

pub async fn expand_topic(topic: &str) -> Result<NodeData> {
    let ollama = Ollama::default();
    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<NodeData>()));
    let prompt = format!(
        "Generate a mind-map entry for '{topic}'. \
         Provide: name, short description (1-2 sentences), and 3-5 closely related concept names."
    );
    let res = ollama
        .generate(
            GenerationRequest::new("qwen2.5:7b".to_string(), prompt)
                .format(format)
                .think(false),
        )
        .await?;
    Ok(serde_json::from_str(&res.response)?)
}
