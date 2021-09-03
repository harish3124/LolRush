#[macro_use]
extern crate clap;

mod cli;

fn main() {
    let contents = cli::get_input();
    println!("{:?}", contents);
}
