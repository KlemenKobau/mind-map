use anyhow::Result;

use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let req = GenerationRequest::new("qwen3.5:4b".to_string(), "Why is the sky blue?").think(false);
    let res = ollama.generate(req).await?;

    println!("{}", res.response);

    Ok(())
}
