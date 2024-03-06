// NOTE:
// temp = (temp * 9 / 5) + 32; for Celsius
// temp = ((temp - 32) * 5) / 9; for Farenheit

// use std::io;

use std::io;

fn main() {
    println!("****** Temperature Converter ******");
    println!("Enter the unit measure: C or F");

    let mut unit: String = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit = unit.trim();

    println!("The value of unit is {unit}");

    if unit == "C" {
        println!("Please enter the temp in C");
        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let mut temp: u32 = temp.trim().parse().unwrap_or(1);
        temp = (temp * 9 / 5) + 32;
        println!("The temperature in Celsius is {temp}");
    } else if unit == "F" {
        println!("Please enter the temp in F");
        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let mut temp: u32 = temp.trim().parse().unwrap_or(1);
        temp = ((temp - 32) * 5) / 9;
        println!("The temperature in Celsius is {temp}");
    }
}
