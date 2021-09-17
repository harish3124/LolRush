#[macro_use]
extern crate clap;
extern crate colorful;

mod cli;
pub use cli::*;

mod gradline;
use gradline::GradLine;
