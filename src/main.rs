use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    return input;
}

fn main() {
    let mut run = true;
    let mut user_input;

    while run {
        println!("number?");
        user_input = get_input();
        println!("user input: {}", user_input);

        if user_input == "end" {
            run = false;
        }
    }
}
