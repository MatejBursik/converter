use crate::get_input::get_input;

pub fn fahrenheit_to_celsius() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input temperature in fahrenheit:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // (value - 32.0) * 5.0 / 9.0
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} fahrenheit -> {} celsius", parsed_number, (parsed_number - 32.0) * 5.0 / 9.0);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}

pub fn celsius_to_fahrenheit() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input temperature in celsius:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value * 9.0 / 5.0 + 32.0
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} celsius -> {} fahrenheit", parsed_number, parsed_number * 9.0 / 5.0 + 32.0);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}