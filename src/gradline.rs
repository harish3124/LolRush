// use colorful::{Colorful, RGB};
use colored::*;
use colorgrad::{rainbow, Gradient};
use rand::random;

pub struct GradLine {
    line: String,
    gradient: Gradient,
    offset: f64,
    line_offset: f64,
}

impl GradLine {
    pub fn new() -> GradLine {
        GradLine {
            line: String::new(),
            gradient: rainbow(),
            offset: 0.0,
            line_offset: random(),
        }
    }

    fn colorize(&mut self, current_line: &String) -> String {
        self.line = current_line.to_string();
        self.offset = self.line_offset;

        let mut grad;

        let mut colored_string = String::from("");

        for alpha in self.line.split("") {
            grad = self.gradient.at(self.offset).rgba_u8();

            colored_string += &alpha.truecolor(grad.0, grad.1, grad.2);

            if self.offset <= 1.0 && self.offset >= 0.0 {
                self.offset += 0.01;
            } else {
                self.offset = 0.0;
            };
        }

        if self.line_offset <= 1.0 && self.line_offset >= 0.0 {
            self.line_offset += 0.01;
        } else {
            self.line_offset = 0.0;
        };

        colored_string
    }

    pub fn print(&mut self, current_line: String) {
        let colored_string = self.colorize(&current_line);
        println!("{}", &colored_string);
    }
}
