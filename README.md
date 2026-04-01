# Mind Map

A desktop app that uses a local Ollama LLM to interactively build a force-directed mind map of concepts.

## Requirements

- [Rust](https://rustup.rs/)
- [Ollama](https://ollama.com/) running locally (`ollama serve`) with `qwen2.5:7b` pulled

## Usage

```bash
cargo run
```

Type a topic into the input field and press Enter. The app queries Ollama to generate related concepts, adds them as nodes, and connects them with edges. Click any unexpanded node to grow the graph further. Drag nodes to pin them in place.

## Features

- Force-directed graph layout (Fruchterman-Reingold physics via `fdg-sim`)
- LLM-powered topic expansion with structured JSON output
- Pan and zoom the viewport
- Drag nodes to pin positions
- Sidebar showing selected node name and description

## MVP

- [x] Create a graph representation
- [x] Ask the user for the search topic
- [x] Create a node, based on user instruction
- [x] Generate connections to related subjects
