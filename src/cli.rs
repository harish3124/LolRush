use clap::ArgMatches;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

use crate::{GradLine, strip_chars};

pub fn get_args<'a>() -> ArgMatches<'a> {
    let matches = clap_app!(lolcat_rs =>
        (version: "1.0")
        (author: "Harish <github.com/harish3124>")
        (about: "An high performance rewrite of lolcat")
        (@arg INPUT: "Sets the input file to use")
    )
    .get_matches();
    return matches;
}

pub fn read_stdin() {
    let stdin = io::stdin();
    let mut line = GradLine::new();

    for current_line in stdin.lock().lines() {
        let current_line = strip_chars(current_line.as_ref().unwrap());
        line.print(current_line.to_string());
    }
}

pub fn read_file(file_name: &str) {
    let file_path = Path::new(file_name);
    let file = File::open(file_path).expect("Unable to open file !");
    let reader = BufReader::new(file);

    let mut line = GradLine::new();

    for current_line in reader.lines() {
        let current_line = strip_chars(current_line.as_ref().unwrap());
        line.print(current_line.to_string());
    }
}
