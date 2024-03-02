use std::io;

fn main() {
    println!("****** Data types ******");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    // can also access with . notation
    let five_hundred = tup.0;
    println!("This is 500 {five_hundred}");
    let five_hundred = tup.1;
    println!("This is 6.4 {five_hundred}");
    let five_hundred = tup.2;
    println!("This is 1 {five_hundred}");

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // accessing single elements
    let first = a[0];
    let second = a[1];
    let third = a[2];
    println!("Values are {first} {second} {third}");
    // accessing all elements
    for value in a {
        println!("Value is {value}");
    }


    // accessing elements past the lenght (overflowing)
    println!("Please enter any array index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
