fn calculate_fibonacci(number: u32) -> u32 {
    match number {
        0 => 0,
        1 => 1,
        _ => calculate_fibonacci(number -1) + calculate_fibonacci(number - 2),

    }
}

fn main() {
    println!("****** nth Fibonacci generator ******");
    let n = 10;
    let fibonacci_number = calculate_fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, fibonacci_number);
}
