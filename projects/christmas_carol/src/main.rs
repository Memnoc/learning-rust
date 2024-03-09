enum Gift {
    Partridge,
    TurtleDoves,
    FrenchHens,
    CallingBirds,
    GoldenRings,
    GeeseALaying,
    SwansASwimming,
    MaidsAMilking,
    LadiesDancing,
    LordsALeaping,
    PipersPiping,
    DrummersDrumming,
}

impl Gift {
    fn verses(&self) -> &'static str {
        match self {
            Gift::Partridge => "a Partridge in a Pear Tree",
            Gift::TurtleDoves => "two Turtle Doves, and",
            Gift::FrenchHens => "three French Hens,",
            Gift::CallingBirds => "four Calling Birds,",
            Gift::GoldenRings => "five Golden Rings,",
            Gift::GeeseALaying => "six Geese a Laying,",
            Gift::SwansASwimming => "seven Swans a Swimming,",
            Gift::MaidsAMilking => "eight Maids a Milking,",
            Gift::LadiesDancing => "nine Ladies Dancing,",
            Gift::LordsALeaping => "ten Lords a Leaping,",
            Gift::PipersPiping => "eleven Pipers Piping,",
            Gift::DrummersDrumming => "twelve Drummers Drumming,",
        }
    }
}

fn days(day: usize) -> &'static str {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eight",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => unreachable!(),
    }
}
fn main() {
    println!("***** Christmas carol ******");
    let gifts = [
        Gift::Partridge,
        Gift::TurtleDoves,
        Gift::FrenchHens,
        Gift::CallingBirds,
        Gift::GoldenRings,
        Gift::GeeseALaying,
        Gift::SwansASwimming,
        Gift::MaidsAMilking,
        Gift::LadiesDancing,
        Gift::LordsALeaping,
        Gift::PipersPiping,
        Gift::DrummersDrumming,
    ];

    for day in 1..=12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            days(day)
        );
        for gift in (0..day).rev() {
            println!("{}", gifts[gift].verses());
        }
        println!();
    }
}
