use day1;
use day2;
use day3;

fn main() {
    println!("Advent of Code 2023!");

    //take input of what day to use

    //run that day's code
    //match over the input

    let input = match std::env::args().nth(1) {
        Some(value) => match value.parse::<u8>() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number between 1 and 25");
                return;
            }
        },
        None => {
            println!("Please enter a number between 1 and 25");
            return;
        }
    };

    match input {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),

        _ => println!("No code for that day yet!"),
    }
}
