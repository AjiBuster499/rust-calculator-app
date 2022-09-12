use app::App;
use calculator::Calculator;

extern crate eframe;

mod app;
mod calculator;

fn main() {
    let calculator = Calculator::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, calculator))),
    )
}
