#[allow(dead_code)]

fn main() {
    println!("****** Ownership ******");
    {
        // scope example
        // not valid here
        let s = "Hello"; // valid here
                         // valid here too (in scope)
    } // scope over and s no longer valid

    {
        // String literal you can mutate
        let mut s = String::from("Hello");
        s.push_str(", world");
        println!("{}", s);
    }
}
