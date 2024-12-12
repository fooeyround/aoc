#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use chrono::Datelike;
use clap::Parser;
use seq_macro::seq;
use std::time::{Duration, Instant};
use std::{fs::File, io::Read};

seq!(N in 01..=12 {
    mod d~N;
});

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long, action)]
    part_two_only: bool,
}

pub fn get_input(day: u8) -> String {
    let mut fine = File::open(format!("puzzle_input/day{:02}.in", day)).expect("Day input missing");
    let mut strin = String::new();
    fine.read_to_string(&mut strin)
        .expect("Failed to read puzzle input for day {day}");
    return strin;
}

pub fn get_display_time(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        return format!("{}secs", duration.as_secs());
    }
    if duration.as_millis() > 0 {
        return format!("{}ms", duration.as_millis());
    }
    if duration.as_micros() > 0 {
        return format!("{}μs", duration.as_micros());
    }
    return format!("{}ns", duration.as_nanos());
}

fn main() {
    println!("🎄Advent of Code 2024!🎄");

    let cli = Cli::parse();
    //take input of what day to use

    let day = cli.day.unwrap_or_else(|| {
        if chrono::Utc::now().month() == 12 {
            chrono::Utc::now().day() as u8
        } else {
            panic!("Not December, Please Choose a Day!")
        }
    });

    if cli.part_two_only {
        println!("Running Only Part Two!");
    }

    seq!(N in 01..=12 {
        match day {
            #(
                N => {
                    let input = get_input(N);
                    if !cli.part_two_only {
                        let initial_time = Instant::now();
                        let output = d~N::solve1(&input);
                        println!("🎄Part 1 ({}): {}", get_display_time(initial_time.elapsed()), output);
                    }
                    let initial_time = Instant::now();
                    let output = d~N::solve2(&input);
                    println!("🎄Part 2 ({}): {}", get_display_time(initial_time.elapsed()), output);
                }
            )*
            _ => panic!("No code for that day yet!"),
        }
    });
}
