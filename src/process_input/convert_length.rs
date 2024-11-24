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

pub fn feet_to_cm() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input length in feet:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value * 30.48
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} feet -> {} cm", parsed_number, parsed_number * 30.48);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}

pub fn cm_to_feet() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input length in cm:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value / 30.48
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} cm -> {} feet", parsed_number, parsed_number / 30.48);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}

pub fn feet_to_m() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input length in feet:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value * 0.3048
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} feet -> {} m", parsed_number, parsed_number * 0.3048);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}

pub fn m_to_feet() {
    let mut run = true;
    let mut value;
    println!("Input 'exit' to leave this functionality");

    while run {
        println!("Input length in m:");
        value = get_input();

        if value == "exit" {
            run = false;
        } else {
            // value / 0.3048
            match value.parse::<f32>() {
                Ok(parsed_number) => {
                    println!("{} m -> {} feet", parsed_number, parsed_number / 0.3048);
                }
                Err(_) => {
                    println!("Invalid number entered!");
                }
            }
        }
    }
}