//! This example project for `nodui` is a simple math expression visual editor.

#![allow(unreachable_pub)]

mod app;
mod graph;

use app::App;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Visual Math",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
