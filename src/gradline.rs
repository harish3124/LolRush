use colorful::core::StrMarker;
use colorgrad::Gradient;

pub struct GradLine<'a> {
    pub line: &'a String,
    gradient: Gradient,
    offset: u8,
}

impl GradLine<'_> {
    pub fn new(gradient_name: &str) -> GradLine {
        unimplemented!();
    }

    // pub fn set_line<'a>(&mut self, line: String) {
    //     self.line = &line;
    // }
}

impl StrMarker for GradLine<'_> {
    fn to_str(&self) -> String {
        String::from(self.line)
    }
}
