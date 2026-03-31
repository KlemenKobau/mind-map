mod app;
mod graph;
mod layout;
mod node;
mod ollama;

fn main() {
    // Must be set before GTK/GDK initializes (Wayland compositor incompatibility on Linux)
    unsafe { std::env::set_var("GDK_BACKEND", "x11") };
    dioxus::launch(app::App);
}
