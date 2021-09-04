use lolcat_rs::*;

fn main() {
    let input = get_args();

    match input.value_of("INPUT") {
        Some(file_name) => read_file(file_name),
        None => read_stdin(),
    };
}

