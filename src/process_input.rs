pub fn process_input(input: String) -> bool {
    match input.as_str() {
        "1" => println!("test 1"),
        "exit" => return false,
        _ => return true
    };
    return true;
}
