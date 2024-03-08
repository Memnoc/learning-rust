fn calculate_fibonacci(number: i32) -> i32 {
    if number <= 1 {
        number
    } else {
        calculate_fibonacci(number - 1) + calculate_fibonacci(number - 2)
    }
}

fn main() {
    println!("****** nth Fibonacci generator ******");
    let n: i32 = 10;
    let fibonacci_number = calculate_fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, fibonacci_number);
}
