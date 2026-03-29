use anyhow::Result;

use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::tools::implementations::{Calculator, DDGSearcher, Scraper};
use ollama_rs::{
    Ollama, coordinator::Coordinator, generation::completion::request::GenerationRequest,
    models::ModelOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let history = vec![];

    let mut coordinator = Coordinator::new(ollama, "qwen3.5:4b".to_string(), history)
        .options(ModelOptions::default())
        .add_tool(Scraper {})
        .add_tool(DDGSearcher::new())
        .add_tool(Calculator {});

    let resp = coordinator
        .think(false)
        .chat(vec![ChatMessage::user(
            "What do you know about destiny the political streamer, search online".to_string(),
        )])
        .await
        .unwrap();

    println!("{}", resp.message.content);

    Ok(())
}
