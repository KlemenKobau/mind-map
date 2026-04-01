mod app;
mod graph;
mod layout;
mod node;
mod ollama;

fn main() {
    // Must be set before GTK/GDK initializes (Wayland compositor incompatibility on Linux)
    // needed to be able to resize the window
    unsafe { std::env::set_var("GDK_BACKEND", "x11") };
    // Disable WebKit DMA-BUF renderer (GBM buffer creation fails on some Linux GPU drivers)
    // another workaround for wayland
    unsafe { std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1") };
    dioxus::launch(app::App);
}
