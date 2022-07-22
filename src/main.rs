fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "dnd app",
        native_options,
        Box::new(|cc| Box::new(dnd_app::TemplateApp::new(cc))),
        
    );
}