use app::App;

extern crate eframe;

mod app;
mod calculator;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
