#[macro_use]
extern crate clap;
extern crate colorful;

mod cli;
pub use cli::*;

mod gradline;
use gradline::GradLine;

mod strip_ansi_chars;
use strip_ansi_chars::strip_chars;

