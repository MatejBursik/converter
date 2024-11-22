mod convert_length;
mod convert_temperature;

use convert_length::*;
use convert_temperature::*;

pub fn process_input(input: String) -> bool {
    match input.as_str() {
        "0" => return false,
        "1" => inch_to_cm(),
        "2" => cm_to_inch(),
        "9" => fahrenheit_to_celsius(),
        "10" => celsius_to_fahrenheit(),
        _ => return true
    };
    return true;
}
