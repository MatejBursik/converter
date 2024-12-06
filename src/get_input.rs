use std::io;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    return input;
}