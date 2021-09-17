use colorful::{Colorful, RGB};
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

    pub fn print(&mut self, current_line: String) {
        self.line = current_line;
        let mut grad = self.gradient.at(self.offset).rgba_u8();

        self.offset = self.line_offset;

        for alpha in self.line.split("") {
            print!("{}", alpha.color(RGB::new(grad.0, grad.1, grad.2)));

            if self.offset <= 1.0 && self.offset >= 0.0 {
                self.offset += 0.01;
            } else {
                self.offset = 0.0;
            };

            grad = self.gradient.at(self.offset).rgba_u8();
        }
        println!("");

        if self.line_offset <= 1.0 && self.line_offset >= 0.0 {
            self.line_offset += 0.01;
        } else {
            self.line_offset = 0.0;
        };

        // println!(
        //     "{}",
        //     self.line[..]
        //         .color(RGB::new(grad.0, grad.1, grad.2))

        // );
    }
}
