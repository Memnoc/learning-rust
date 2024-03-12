fn main() {
    println!("****** References and Borrowing ******");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}
