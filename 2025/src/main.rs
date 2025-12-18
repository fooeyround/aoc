// #[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use chrono::Datelike;
use clap::Parser;
use cli_table::{Table, WithTitle, print_stdout};
use seq_macro::seq;
use std::time::{Duration, Instant};
use std::{fs::File, io::Read};

pub type PartFn = fn(&str) -> String;

pub struct Day {
    pub day: u8,
    pub part1: PartFn,
    pub part2: PartFn,
}

inventory::collect!(Day);

seq!(N in 01..=02 {
    mod d~N;
    inventory::submit! {
        Day {
            day: N,
            part1: d~N::p1,
            part2: d~N::p2,
        }
    }
});

#[derive(Table)]
struct DayInfo {
    #[table(title = "Day & Part")]
    day_and_part: String,
    #[table(title = "Time")]
    time: String,
    #[table(title = "Output")]
    output: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long, action)]
    part_two_only: bool,
    ///Enable test mode (Using test input instead of real input)
    #[arg(short, long, action)]
    test_mode: bool,
}

/// returns (Example, Real)
pub fn get_input(day: u8, test_mode: bool) -> String {
    let mut input = String::new();

    let mut file = File::open(format!(
        "puzzle_input/d{:02}.{}",
        day,
        if test_mode { "ex" } else { "in" }
    ))
    .expect("Day input missing");
    file.read_to_string(&mut input)
        .expect("Failed to read puzzle input for day");
    return input;
}

pub fn get_display_time(duration: Duration) -> String {
    if duration.as_secs() >= 1 {
        return format!("{}secs", duration.as_secs());
    }
    if duration.as_millis() >= 1 {
        return format!("{}ms", duration.as_millis());
    }
    if duration.as_micros() >= 1 {
        return format!("{}μs", duration.as_micros());
    }
    return format!("{}ns", duration.as_nanos());
}

/*
pub fn run_day(day: u8, day_impl: &impl Day, part_two_only: bool) {
    let input = get_input(day);
    if part_two_only {
        let initial_time = Instant::now();
        let output = (*day_impl).p1(&input);
        println!(
            "🎄Part 1 ({}): {}",
            get_display_time(initial_time.elapsed()),
            output
        );
    }
    let initial_time = Instant::now();
    let output = (*day_impl).p2(&input);
    println!(
        "🎄Part 2 ({}): {}",
        get_display_time(initial_time.elapsed()),
        output
    );
}
*/

fn run_timed<T>(func: impl Fn() -> T) -> (Duration, T) {
    let initial_time = Instant::now();
    let output = func();
    return (initial_time.elapsed(), output);
}

fn main() {
    println!("🎄Advent of Code 2025!🎄");

    let cli = Cli::parse();

    if cli.part_two_only {
        println!("Running Only Part Two!");
    }

    let day_impls = inventory::iter::<Day>.into_iter().filter(|d| {
        if let Some(day) = cli.day {
            day == d.day
        } else {
            true
        }
    });
    let mut data: Vec<DayInfo> = Vec::new();

    for day_impl in day_impls {
        let input = get_input(day_impl.day, cli.test_mode);
        println!("🎄Testing Day {}", day_impl.day);
        if !cli.part_two_only {
            let (time, output) = run_timed(|| (day_impl.part1)(&input));
            data.push(DayInfo {
                day_and_part: format!("{:02} p1", day_impl.day),
                time: get_display_time(time),
                output: output,
            });
        }
        let (time, output) = run_timed(|| (day_impl.part2)(&input));
        data.push(DayInfo {
            day_and_part: format!("{:02} p2", day_impl.day),
            time: get_display_time(time),
            output: output,
        });
    }

    if cli.test_mode {
        println!("🧪Test Mode Results");
    } else {
        println!("🎄Results");
    }
    print_stdout(data.with_title()).expect("Printable results");

    // println!("🎄Part 2");
    // println!("🎄Test ({}): {}", get_display_time(test_dir), test_output);
    // println!("🎄Real ({}): {}", get_display_time(real_dir), real_output);
}
