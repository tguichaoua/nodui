//! This example project for `nodui` is a playground to test different editor settings.

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Playground",
        native_options,
        Box::new(|cc| Ok(Box::new(playground::App::new(cc)))),
    )
}
