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

This is a **Dioxus desktop GUI app** that uses a local Ollama LLM to interactively build a force-directed mind-map of concepts. It is not a CLI.

### Module overview

- [src/main.rs](src/main.rs) — entry point; sets Linux Wayland env vars (`GDK_BACKEND=x11`, `WEBKIT_DISABLE_DMABUF_RENDERER=1`) then launches the Dioxus app
- [src/app.rs](src/app.rs) — the entire UI: Dioxus signals for reactive state, SVG-based graph rendering, pan/zoom, node drag-and-drop, and calls to `expand_topic()`
- [src/ollama.rs](src/ollama.rs) — `expand_topic(topic)` sends a structured JSON generation request to `qwen2.5:7b` via `ollama.generate()` and deserializes the response into `NodeData`
- [src/graph.rs](src/graph.rs) — `NodeData` struct (name, description, related vec) with `JsonSchema` + `Deserialize` derives for structured LLM output
- [src/node.rs](src/node.rs) — `UiNode` struct and color helper functions (`node_fill`, `node_stroke`) for node rendering state
- [src/layout.rs](src/layout.rs) — empty, reserved

### Key dependencies

- **dioxus** (desktop feature) — reactive UI framework; renders SVG via a WebView
- **fdg-sim** — Fruchterman-Reingold force-directed graph physics; drives node positions each 16ms tick
- **ollama-rs** — LLM client for local Ollama; uses `FormatType::StructuredJson` with a `JsonStructure::new::<NodeData>()` schema (not `Coordinator`)
- **schemars** — derives the JSON schema from `NodeData` for structured LLM output
- **tokio** — async runtime (used inside `spawn` calls from Dioxus event handlers)
- **anyhow** — error propagation

### Data flow

1. User types a topic → `expand_topic()` is called asynchronously
2. Ollama returns a `NodeData` (name, description, 3–5 related concept names)
3. A new `fdg-sim` node is added for the topic and each related concept; edges are created between them
4. The simulation loop (16ms) runs physics ticks and updates SVG positions via Dioxus signals
5. Clicking an unexpanded node calls `expand_topic()` again to grow the graph
6. Dragging pins a node (freezes it in the physics simulation); small drags (< 5px) are treated as clicks
