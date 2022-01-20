use colored::*;
use colorgrad::{rainbow, Gradient};
use rand::random;
use unicode_segmentation::UnicodeSegmentation;

pub struct GradLine {
    line: String,
    gradient: Gradient,
    line_offset: f64,
}

impl GradLine {
    pub fn new() -> GradLine {
        GradLine {
            line: String::new(),
            gradient: rainbow(),
            line_offset: random(),
        }
    }

    pub fn print(&mut self, current_line: String) {
        self.line = current_line.to_string();
        let mut offset = self.line_offset;

        let mut grad;

        let grapheme_vec = self.split_grapheme();

        for grapheme in grapheme_vec {
            grad = self.gradient.at(offset).rgba_u8();

            print!("{}", grapheme.truecolor(grad.0, grad.1, grad.2));

            if offset <= 1.0 && offset >= 0.0 {
                offset += 0.01;
            } else {
                offset = 0.0;
            };
        }

        if self.line_offset <= 1.0 && self.line_offset >= 0.0 {
            self.line_offset += 0.01;
        } else {
            self.line_offset = 0.0;
        };

        println!();
    }

    fn split_grapheme(&self) -> Vec<&str> {
        let graphemes = &self.line.graphemes(true).collect::<Vec<&str>>();
        graphemes.to_vec()
    }
}
