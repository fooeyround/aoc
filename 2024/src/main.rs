use chrono::Datelike;
use clap::{Parser, Subcommand};
use seq_macro::seq;
use std::{fs::File, io::Read};

mod d1;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,
}

pub fn get_input(day: u8) -> Vec<String> {
    let mut fine =
        File::open(format!("puzzle_input/day{}.in", day)).expect("Day {day} input missing");
    let mut strin = String::new();
    fine.read_to_string(&mut strin)
        .expect("Failed to read puzzle input for day {day}");
    return strin.split("\n").map(|f| f.to_owned()).collect();
}

fn main() {
    println!("Advent of Code 2024!");

    let cli = Cli::parse();
    //take input of what day to use

    let day = cli.day.unwrap_or_else(|| {
        if chrono::Utc::now().month() == 12 {
            chrono::Utc::now().day() as u8
        } else {
            panic!("Not December, Please Choose a Day!")
        }
    });

    let (p1, p2) = seq!(N in 1..=1 {
        match day {
            // Expands to Variant64, Variant65, ...
            #(
                N => {
                    let input = get_input(1);
                    (d~N::solve1(&input), d~N::solve2(&input))
                }
            )*
            _ => panic!("No code for that day yet!"),
        }
    });

    println!("🎄Part 1: {}", p1);
    println!("🎄Part 2: {}", p2);
}
