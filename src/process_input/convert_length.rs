use crate::get_input::get_input;

pub fn inch_to_cm() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input length in inches:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value * 2.54
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} inch -> {} cm", parsed_number, parsed_number * 2.54);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}

pub fn cm_to_inch() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input length in cm:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value / 2.54
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} cm -> {} inch", parsed_number, parsed_number / 2.54);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}