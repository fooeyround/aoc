use std::collections::HashMap;

use common::get_input;

pub fn run() {
    let input = get_input();

    println!("Day 6");
    part_one(&input);

    part_two(&input);
}

fn parse_part_one(input: &Vec<String>) -> HashMap<u32, u32> {
    input
        .get(0)
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .zip(
            input
                .get(0)
                .unwrap()
                .split_once(" ")
                .unwrap()
                .1
                .trim()
                .split_whitespace(),
        )
        .map(|x| (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()))
        .collect()
}

fn parse_part_two(input: &Vec<String>) -> (u64, u64) {
    let time: u64 = input
        .get(0)
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance: u64 = input
        .get(1)
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();

    return (time, distance);
}

fn get_possible_charge_times(parsed: (u64, u64)) -> Vec<u64> {
    let mut distances: Vec<u64> = Vec::new();

    for charge_time in 0..parsed.0 {
        let time_left = parsed.0 - charge_time;
        let this_distance = charge_time * time_left;
        if this_distance >= parsed.1 {
            distances.push(charge_time);
        }
    }
    return distances;
}

fn part_one(input: &Vec<String>) {
    let parsed = parse_part_one(input);

    let answer = parsed
        .iter()
        .map(|val| get_possible_charge_times((*val.0 as u64, *val.1 as u64)))
        .fold(
            0,
            |acc, val| {
                if acc == 0 {
                    val.len()
                } else {
                    acc * val.len()
                }
            },
        );

    // }

    println!("Part One: {}", answer);
}

fn part_two(input: &Vec<String>) {
    let parsed: (u64, u64) = parse_part_two(input);
    println!("Part 2: {:?}", get_possible_charge_times(parsed).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test_input.txt")
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        part_one(&input);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test_input.txt")
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        part_two(&input);
    }
}
