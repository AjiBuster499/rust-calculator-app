use eframe::egui;

use crate::calculator::Calculator;

#[derive(Default)]
pub(crate) struct App {
    calculator: Calculator,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            calculator: Calculator::new(),
        }
    }
}

// TODO: Layout for Calculator
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                // Exit Button
                frame.close();
            }
            ui.label(&self.calculator.equation);
            // TODO: Clean this up
            if ui.button("0").clicked() {
                self.calculator.push_to_equation("0");
            }
            if ui.button("1").clicked() {
                self.calculator.push_to_equation("1");
            }
            if ui.button("2").clicked() {
                self.calculator.push_to_equation("2");
            }
            if ui.button("3").clicked() {
                self.calculator.push_to_equation("3");
            }
            if ui.button("4").clicked() {
                self.calculator.push_to_equation("4");
            }
            if ui.button("5").clicked() {
                self.calculator.push_to_equation("5");
            }
            if ui.button("6").clicked() {
                self.calculator.push_to_equation("6");
            }
            if ui.button("7").clicked() {
                self.calculator.push_to_equation("7");
            }
            if ui.button("8").clicked() {
                self.calculator.push_to_equation("8");
            }
            if ui.button("9").clicked() {
                self.calculator.push_to_equation("9");
            }
            if ui.button("+").clicked() {
                self.calculator.push_to_equation(" + ");
            }
            if ui.button("-").clicked() {
                self.calculator.push_to_equation(" - ");
            }
            if ui.button("*").clicked() {
                self.calculator.push_to_equation(" * ");
            }
            if ui.button("/").clicked() {
                self.calculator.push_to_equation(" / ");
            }
            if ui.button("=").clicked() {
                self.calculator.push_to_equation(" )");
                let answer = self.calculator.calculate();
                self.calculator.push_to_equation(&answer);
            }
        });
    }
}
