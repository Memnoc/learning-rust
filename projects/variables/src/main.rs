
fn main() {
    println!("******* VARIABLES & MUTABILITY **********");

    let mut x = 5;
    println!("This is the value of x {x}");

    x = 7;
    println!("This is the value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is the value of the constant {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 5;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y in the outer scope is {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("This is the value of spaces: {spaces}");
}
