use eframe::egui::Vec2;

mod ui;
mod command;
mod request;
mod config;

fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(300., 100.)),
        ..Default::default()
    };
    eframe::run_native(Box::new(ui::App::default()), options);
}