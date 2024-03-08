// NOTE:
// temp = (temp * 9 / 5) + 32; for Celsius
// temp = ((temp - 32) * 5) / 9; for Farenheit

use std::io;

fn read_temperature(prompt: &str) -> f32 {
    println!("{prompt}");
    let mut temp_str = String::new();
    io::stdin()
        .read_line(&mut temp_str)
        .expect("Failed to read line");
    match temp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            panic!()
        }
    }
}

fn convert_temperature(temp: f32, unit: &str) -> f32 {
    match unit {
        "C" => (temp * 9.0 / 5.0) + 32.0,
        "F" => (temp - 32.0) * 5.0 / 9.0,
        _ => 0.0, // invalid unit
    }
}

fn main() {
    println!("****** Temperature Converter ******");
    println!("Enter the unit measure: C or F");

    let mut unit: String = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit = unit.trim().to_uppercase(); // allows for lower case input

    match unit.as_str() {
        "C" => {
            let temp = read_temperature("Please enter the temperature in C: ");
            let converted = convert_temperature(temp, "C");
            println!("The temperature in Farenheit is {converted}");
        }
        "F" => {
            let temp = read_temperature("Please enter the temperature in F: ");
            let converted = convert_temperature(temp, "F");
            println!("The temperature in Celsius is {converted}");
        }
        _ => {
            println!("Please enter a valid unit (C or F)")
        }
    }
}
