use colorful::{Colorful, RGB};

use crate::GradLine;

pub fn colorize(current_line: &GradLine) {
    println!("{}", current_line.color(RGB::new(0, 0, 0)));
}
