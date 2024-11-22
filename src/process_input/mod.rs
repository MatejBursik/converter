mod convert_length;

use convert_length::*;

pub fn process_input(input: String) -> bool {
    match input.as_str() {
        "0" => return false,
        "1" => inch_to_cm(),
        "2" => cm_to_inch(),
        _ => return true
    };
    return true;
}
