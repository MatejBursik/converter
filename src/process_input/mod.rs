mod convert_length;
mod convert_temperature;

use convert_length::*;
use convert_temperature::*;

pub fn process_input(input: String) -> bool {
    match input.as_str() {
        "0" => return false,
        "1" => inch_to_cm(),
        "2" => cm_to_inch(),
        "3" => feet_to_cm(),
        "4" => cm_to_feet(),
        "5" => feet_to_m(),
        "6" => m_to_feet(),
        "9" => fahrenheit_to_celsius(),
        "10" => celsius_to_fahrenheit(),
        _ => return true
    };
    return true;
}
