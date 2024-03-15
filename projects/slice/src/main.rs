// little program to return words separated by a space

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn main() {
    println!("****** Slice Type ******");
    let word = String::from("hello world");

    let first = first_word(&word);

    println!("the first word is: {}", first);
}
