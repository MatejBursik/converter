mod get_input;
mod process_input;

use get_input::get_input;
use process_input::process_input;

fn main() {
    let mut run = true;
    let mut user_input;

    while run {
        println!("0) Exit");
        println!("\nLength");
        println!("1) inch -> cm");
        println!("2) cm -> inch");
        println!("3) feet -> cm");
        println!("4) cm -> feet");
        println!("5) feet -> m");
        println!("6) m -> feet");
        println!("7) mile (statute) -> km");
        println!("8) km -> mile (statute)");
        println!("\nTemperature");
        println!("9) fahrenheit -> celsius");
        println!("10) celsius -> fahrenheit");
        println!("\nChoose the functionality (enter its number):");

        user_input = get_input();
        run = process_input(user_input);
    }
}
