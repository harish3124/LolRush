use strip_ansi_escapes::strip;

pub fn strip_chars(input: &String) -> String {
    let tmp = input.as_bytes();
    let tmp = strip(&tmp).unwrap();
    String::from_utf8(tmp).unwrap()
}
