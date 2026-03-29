# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build

# Build (release)
cargo build --release

# Run
cargo run

# Check (fast compile check without producing binary)
cargo check

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Lint
cargo clippy

# Format
cargo fmt
```

Ollama must be running locally for the app to work (`ollama serve`).

## Architecture

This is a Rust CLI that uses a local Ollama LLM to interactively build a mind-map (graph) of concepts.

**Planned MVP goals (from README):**
1. Accept user input for a topic
2. Create a graph node for the topic
3. Generate connections to related subjects

**Current state:** Proof-of-concept in [src/main.rs](src/main.rs) — hardcoded query via the `ollama-rs` `Coordinator`, which manages tool calls and chat history.

### Key dependencies

- **ollama-rs** — LLM client wrapping a local Ollama server; uses the `Coordinator` abstraction for multi-turn, tool-augmented chat
- **tokio** — async runtime (`#[tokio::main]`)
- **anyhow** — error propagation

### LLM setup

The `Coordinator` is the main abstraction: it wraps an `Ollama` client, a model name (`qwen3.5:4b`), and a chat history. Tools (Scraper, DDGSearcher, Calculator) are registered on it and can be invoked by the model during a `.chat()` call. `.think(false)` disables chain-of-thought output.

```rust
let mut coordinator = Coordinator::new(ollama, "qwen3.5:4b".to_string(), history)
    .add_tool(Scraper {})
    .add_tool(DDGSearcher::new())
    .add_tool(Calculator {});
```
