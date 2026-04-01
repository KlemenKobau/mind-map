use anyhow::Result;
use ollama_rs::{
    Ollama,
    coordinator::Coordinator,
    generation::chat::ChatMessage,
    generation::completion::request::GenerationRequest,
    generation::parameters::{FormatType, JsonStructure},
    generation::tools::implementations::{DDGSearcher, Scraper},
};

use crate::graph::NodeData;

pub async fn expand_topic(topic: &str) -> Result<NodeData> {
    // Step 1: Research the topic using web search and scraping tools
    let mut coordinator = Coordinator::new(Ollama::default(), "qwen2.5:7b".to_string(), vec![])
        .add_tool(DDGSearcher::new())
        .add_tool(Scraper::new());

    let research = coordinator
        .chat(vec![ChatMessage::user(format!(
            "Search for information about '{topic}'. Summarize: what it is, \
             a brief 1-2 sentence description, and 3-5 closely related concepts."
        ))])
        .await?;

    let research_text = &research.message.content;

    // Step 2: Convert the research into structured NodeData
    let b = Box::new(JsonStructure::new::<NodeData>());
    let format = FormatType::StructuredJson(b);
    let prompt = format!(
        "Based on this research:\n{research_text}\n\n\
         Generate a mind-map entry for '{topic}'. \
         Provide: name, short description (1-2 sentences), and 3-5 closely related concept names."
    );
    let res = Ollama::default()
        .generate(
            GenerationRequest::new("qwen2.5:7b".to_string(), prompt)
                .format(format)
                .think(false),
        )
        .await?;
    Ok(serde_json::from_str(&res.response)?)
}
