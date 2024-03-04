fn main() {
    println!("****** Control flow ******");

    let number = 6;
    let mut counter = 0;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut while_counter = 3;

    while while_counter != 0 {
        println!("{number}");

        while_counter -= 1;
    }

    println!("LIFTOFF!!");

    // for loops
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for while_counter in (1..4).rev() {
        println!("{while_counter}!");
    }
    println!("LIFTOFF TWO");
}
