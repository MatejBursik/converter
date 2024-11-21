mod get_input;
mod process_input;

use get_input::get_input;
use process_input::process_input;

fn main() {
    let mut run = true;
    let mut user_input;

    while run {
        println!("Length");
        println!("1) inch -> cm");
        println!("2) cm -> inch");
        println!("\nTemperature");
        println!("3) fahrenheit -> celsius");
        println!("4) celsius -> fahrenheit");
        println!("\nChoose the action (enter its number):");

        user_input = get_input();
        run = process_input(user_input);
    }
}
