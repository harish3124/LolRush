#[macro_use]
extern crate clap;
extern crate colorful;


mod cli;
pub use cli::get_args;

mod gradline;
use gradline::GradLine;

mod colorize;
use colorize::colorize;
