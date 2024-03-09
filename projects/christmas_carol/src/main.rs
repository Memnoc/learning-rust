fn days(day: u32) -> &'static str {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eight",
        9 => "nineth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelveth",
        _ => unreachable!(),
    }
}

fn verses() {
    for day in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love sent to me:",
            days(day)
        );
        match day {
            12 => println!("Twelves Drummers Drumming"),
            11 => println!("Eleven Pipers Piping"),
            10 => println!("Ten Lords a Leaping"),
            9 => println!("Nine Ladies Dancing"),
            8 => println!("Eight Maids a Milking"),
            7 => println!("Seven Swans a Swimming"),
            6 => println!("Six Geese a Laying"),
            5 => println!("Five Golden Rings"),
            4 => println!("Four Calling Birds"),
            3 => println!("Three French Hens"),
            2 => println!("Two Turtle Doves, and"),
            1 => println!("A Partridge in a Pear Tree"),
            _ => unreachable!(),
        }
        println!();
    }
}

fn main() {
    println!("***** Christmas carol ******");
    verses();
}
