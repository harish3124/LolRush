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

        let mut escaping = false;

        for grapheme in grapheme_vec {
            if grapheme == "\x1B" {
                print!("\x1B");
                escaping = true;
                continue;
            } else if escaping {
                print!("{}", grapheme);

                escaping = grapheme.len() != 1 || {
                    let c = grapheme.as_bytes()[0];
                    !(b'a'..=b'z').contains(&c) && !(b'A'..=b'Z').contains(&c)
                };
                continue;
            }
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
