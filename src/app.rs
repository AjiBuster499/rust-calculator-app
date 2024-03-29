use crate::calculator::Calculator;
use eframe::egui;

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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Quit").clicked() {
                    // Exit Button
                    frame.close();
                }
                if ui.button("Clear").clicked() {
                    self.calculator.clear();
                }
                if ui.button("Scientific").clicked() {
                    self.calculator.scientific = !self.calculator.scientific;
                }
                ui.label(&self.calculator.display_equation);
            });
            // TODO: Clean this up
            if self.calculator.scientific {
                // Implement Scientific Functions here.
                // TODO: Scientific Buttons
                ui.columns(4, |cols| {
                    // Needs a way to close the parenthesis
                    if cols[0].button("log").clicked() {
                        self.calculator.push_to_equation(" log ( ");
                    }
                    if cols[1].button("ln").clicked() {
                        self.calculator.push_to_equation(" ln ( ");
                    }
                    if cols[2].button("sin").clicked() {
                        // TODO: Sine
                        self.calculator.push_to_equation(" sin ( ");
                    }
                    if cols[3].button("^").clicked() {
                        self.calculator.push_to_equation(" ^ ( ");
                    }
                });
            }
            ui.columns(4, |cols| {
                if cols[0].button("(").clicked() {
                    self.calculator.push_to_equation(" ( ");
                }
                if cols[0].button(")").clicked() {
                    self.calculator.push_to_equation(" )");
                }
                if cols[0].button("7").clicked() {
                    self.calculator.push_to_equation("7");
                }
                if cols[0].button("4").clicked() {
                    self.calculator.push_to_equation("4");
                }
                if cols[0].button("1").clicked() {
                    self.calculator.push_to_equation("1");
                }
                if cols[0].button("0").clicked() {
                    self.calculator.push_to_equation("0");
                }
                if cols[1].button("8").clicked() {
                    self.calculator.push_to_equation("8");
                }
                if cols[1].button("5").clicked() {
                    self.calculator.push_to_equation("5");
                }
                if cols[1].button("2").clicked() {
                    self.calculator.push_to_equation("2");
                }
                if cols[1].button(".").clicked() {
                    self.calculator.push_to_equation(".");
                }
                if cols[2].button("9").clicked() {
                    self.calculator.push_to_equation("9");
                }
                if cols[2].button("6").clicked() {
                    self.calculator.push_to_equation("6");
                }
                if cols[2].button("3").clicked() {
                    self.calculator.push_to_equation("3");
                }
                if cols[2].button("=").clicked() {
                    self.calculator.push_to_equation(" )");
                    self.calculator.calculate();
                }
                if cols[3].button("+").clicked() {
                    self.calculator.push_to_equation(" + ");
                }
                if cols[3].button("-").clicked() {
                    self.calculator.push_to_equation(" - ");
                }
                if cols[3].button("*").clicked() {
                    self.calculator.push_to_equation(" * ");
                }
                if cols[3].button("/").clicked() {
                    self.calculator.push_to_equation(" / ");
                }
                // Is negation possible with our current equation implementation?
                // if cols[3].button("±").clicked() {}
            });
        });
    }
}
