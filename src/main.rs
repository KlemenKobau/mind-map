use anyhow::Result;

use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::parameters::{FormatType, JsonStructure};
use ollama_rs::generation::tools::implementations::{Calculator, DDGSearcher, Scraper};
use ollama_rs::{
    Ollama, coordinator::Coordinator, generation::completion::request::GenerationRequest,
    models::ModelOptions,
};

use crate::graph::NodeData;

mod graph;

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    /*

    let history = vec![];
    let format: FormatType = FormatType::StructuredJson(Box::new(JsonStructure::new::<NodeData>()));


    let mut coordinator = Coordinator::new(ollama, "qwen3.5:4b".to_string(), history)
        .options(ModelOptions::default())
        .add_tool(Scraper {})
        .add_tool(DDGSearcher::new())
        .add_tool(Calculator {})
        .format(format);

    let resp = coordinator
        .think(false)
        .chat(vec![ChatMessage::user(
            "I would like to learn more about cars".to_string(),
        )])
        .await
        .unwrap();

    println!("{}", resp.message.content);
    */

    let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<NodeData>()));
    dbg!(&format);
    let res = ollama
        .generate(
            GenerationRequest::new("qwen2.5:3b".to_string(), "Tell me more about cars")
                .format(format)
                .think(false),
        )
        .await?;

    let resp: NodeData = serde_json::from_str(&res.response)?;

    dbg!(resp);

    Ok(())
}
