use colorful::{Colorful, RGB};
use colorgrad::{rainbow, Gradient};

pub struct GradLine {
    pub line: String,
    pub gradient: Gradient,
    pub offset: f64,
}

impl GradLine {
    pub fn new() -> GradLine {
        GradLine {
            line: String::new(),
            gradient: rainbow(),
            offset: 0.0,
        }
    }

    pub fn print(&mut self, current_line: String) {
        self.line = current_line;
        self.colorize();
    }

    fn get_line(&self) -> &str {
        &self.line[..]
    }

    fn get_grad(&self) -> (u8, u8, u8) {
        let grad = self.gradient.at(self.offset).rgba_u8();

        (grad.0, grad.1, grad.2)
    }

    fn colorize(&self) {
        let grad = self.get_grad();
        println!(
            "{}",
            self.get_line().color(RGB::new(grad.0, grad.1, grad.2))
        );
    }
}
