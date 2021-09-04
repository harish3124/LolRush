use clap::ArgMatches;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn get_args<'a>() -> ArgMatches<'a> {
    let matches = clap_app!(lolcat_rs =>
        (version: "1.0")
        (author: "Harish <harishg3124@gmail.com>")
        (about: "An high performance rewrite of lolcat")
        (@arg INPUT: "Sets the input file to use")
    )
    .get_matches();
    return matches;
}

fn read_stdin() -> Vec<String> {
    let mut contents: Vec<String> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.as_ref().unwrap());
        contents.push(line.unwrap());
    };

    contents
}

fn read_file(file_name: &str) -> Vec<String> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path).expect("Can't open file !");

    let mut contents: Vec<String> = vec![];

    let reader = BufReader::new(file);

    for line in reader.lines() {
        contents.push(line.unwrap());
    }

    contents
}

pub fn get_input() -> Vec<String> {
    let input = get_args();

    match input.value_of("INPUT") {
        Some(file_name) => read_file(file_name),
        None => read_stdin(),
    }
}
