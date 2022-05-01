#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions {
        transparent: true,
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(mark_it_down::TemplateApp::new(cc)))
    );
}
