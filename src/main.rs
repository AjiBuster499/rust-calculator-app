use app::App;

mod app;
mod calculator;
mod operators;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
