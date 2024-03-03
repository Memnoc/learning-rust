fn main() {
    println!("****** Functions ******");

    // call a function
    another_function();

    print_value(5);

    print_label_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of y is: {y}");
}

fn another_function() {
    println!("Another function");
}

fn print_value(x: i32) {
    println!("Value is {x}");
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
