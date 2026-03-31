mod app;
mod graph;
mod layout;
mod node;
mod ollama;

fn main() {
    dioxus::launch(app::App);
}
