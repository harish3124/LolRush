use colorgrad::{rainbow, Gradient};

pub struct GradLine {
    pub line: String,
    gradient: Gradient,
    offset: u8,
}

impl GradLine {
    pub fn new() -> GradLine {
        GradLine {
            line: String::new(),
            gradient: rainbow(),
            offset: 0,
        }
    }

    pub fn get_line(&self) -> &str {
        &self.line[..]
    }
}
