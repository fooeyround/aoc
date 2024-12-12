use cached::proc_macro::cached;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

pub fn parse_input(raw_input: &str) -> Vec<u64> {
    strip_trailing_newline(raw_input)
        .split_whitespace()
        .filter(|f| !f.is_empty())
        .map(|f| f.parse::<u64>().expect("Number"))
        .collect()
}

#[cached]
fn exp(num: u64, iter_count: u64) -> u64 {
    if iter_count == 0 {
        return 1;
    }
    if num == 0 {
        return exp(1, iter_count - 1);
    }
    let digit_count = (num as f64).log10().floor() as u32 + 1;
    if digit_count % 2 == 0 {
        let div = 10u64.pow(digit_count / 2);
        return exp(num / div, iter_count - 1) + exp(num % div, iter_count - 1);
    }

    return exp(num * 2024, iter_count - 1);
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let sum: u64 = input.par_iter().map(|&val| exp(val, 25)).sum();

    return sum.to_string();
}

pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let sum: u64 = input.par_iter().map(|&val| exp(val, 75)).sum();

    return sum.to_string();
}
