#[macro_use]
extern crate clap;

mod cli;
pub use cli::*;

mod gradline;
use gradline::GradLine;
