use itertools::Itertools;
use std::{collections::HashMap, ops::SubAssign};

use common::get_input;

pub fn run() {
    println!("Day 5");

    let input = get_input();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<String>) {
    let parsed = parse_input(input);

    let closest_location = parse_map_part_one(parsed);
    println!("Part 1: {:?}", closest_location);
}

fn part_two(input: &Vec<String>) {
    let parsed = parse_input(input);

    let closest_location = parse_map_part_two(parsed);

    println!("Part 2: {:?}", closest_location);
}


#[derive(Debug, Clone)]
struct SimpleRange {
    start: u64,
    length: u64,
}
impl SimpleRange {
    fn new(start: u64, length: u64) -> SimpleRange {
        SimpleRange { start, length }
    }

    fn contains(&self, value: u64) -> bool {
        self.start <= value && value < self.start + self.length
    }
    fn intersection(&self, other_range: &SimpleRange) -> Option<SimpleRange> {

        //If the ranges do not overlap, return None
        if self.start + self.length < other_range.start || other_range.start + other_range.length < self.start {
            return None;
        }
        let new_start = self.start.max( other_range.start);
        return Some(SimpleRange::new(new_start, (self.length + self.start).min(other_range.length + other_range.start) - new_start));

    }
}

#[derive(Debug, Clone)]
struct RawRangeMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl RawRangeMap {
    fn new(
        destination_range_start: u64,
        source_range_start: u64,
        range_length: u64,
    ) -> RawRangeMap {
        RawRangeMap {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }


    fn contains(&self, value: u64, reverse: bool) -> bool {
        if reverse {
            self.destination_range_start <= value
                && value < self.destination_range_start + self.range_length
        } else {
            self.source_range_start <= value
                && value < self.source_range_start + self.range_length
        }
    }

    fn intersection(&self) -> Option<SimpleRange> {
        //If the ranges do not overlap, return None
        if self.source_range_start + self.range_length < self.destination_range_start
            || self.destination_range_start + self.range_length < self.source_range_start
        {
            return None;
        }
        let new_start = self
            .source_range_start
            .max(self.destination_range_start);
        return Some(SimpleRange::new(
            new_start,
            (self.source_range_start + self.range_length)
                .min(self.destination_range_start + self.range_length)
                - new_start,
        ));
    }

    
}

impl std::ops::Sub for RawRangeMap {
    type Output = Self;
     fn sub(self, other: Self) -> Self::Output {
            RawRangeMap::new(self.destination_range_start - other.destination_range_start, self.source_range_start - other.source_range_start, self.range_length - other.range_length)
     }
}
impl std::ops::Add for RawRangeMap {
    type Output = Self;
     fn add(self, other: Self) -> Self::Output {
            RawRangeMap::new(self.destination_range_start.min(other.destination_range_start), self.source_range_start.min( other.source_range_start), self.range_length.max(other.range_length))
     }
}



///Returns a new hashmap with the keys and values of the one passed swapped.

fn get_map_out_from_input(maps: &Vec<RawRangeMap>, input: u64, reverse: bool) -> u64 {
    //if reverse call the helper or switch case?

    for raw_range_map in maps {
        let matched_value = match reverse {
            false => {
                if raw_range_map.source_range_start <= input
                    && input < raw_range_map.source_range_start + raw_range_map.range_length
                {
                    return input - raw_range_map.source_range_start
                        + raw_range_map.destination_range_start;
                }
            }

            //Handle reversed case
            true => {
                if raw_range_map.destination_range_start <= input
                    && input < raw_range_map.destination_range_start + raw_range_map.range_length
                {
                    return input - raw_range_map.destination_range_start
                        + raw_range_map.source_range_start;
                }
            }
        };
    }

    input
}

fn parse_raw_range_maps(
    map: (&Vec<u64>, &HashMap<String, Vec<(u64, u64, u64)>>),
) -> HashMap<String, Vec<RawRangeMap>> {
    map.1
        .iter()
        .map(|map| {
            let mut map_values: Vec<RawRangeMap> = Vec::new();
            for map_value in map.1 {
                map_values.push(RawRangeMap::new(map_value.0, map_value.1, map_value.2));
            }
            (map.0.clone(), map_values)
        })
        .collect()
}

//The map seems to be missing values
fn parse_map_part_one(map: (Vec<u64>, HashMap<String, Vec<(u64, u64, u64)>>)) -> u64 {
    let seed_list = map.0;
    let maps = map.1;

    let parsed_maps = parse_raw_range_maps((&seed_list, &maps));

    let seeds_with_locations = seed_list
        .iter()
        .map(|seed| {
            //I know it should exist... this still is a pain of unwrap and rigid code.
            let soil =
                get_map_out_from_input(&parsed_maps.get("seed-to-soil").unwrap(), *seed, false);
            let fertilizer =
                get_map_out_from_input(parsed_maps.get("soil-to-fertilizer").unwrap(), soil, false);
            let water = get_map_out_from_input(
                parsed_maps.get("fertilizer-to-water").unwrap(),
                fertilizer,
                false,
            );
            let light =
                get_map_out_from_input(parsed_maps.get("water-to-light").unwrap(), water, false);
            let temperature = get_map_out_from_input(
                parsed_maps.get("light-to-temperature").unwrap(),
                light,
                false,
            );
            let humidity = get_map_out_from_input(
                parsed_maps.get("temperature-to-humidity").unwrap(),
                temperature,
                false,
            );
            let location = get_map_out_from_input(
                parsed_maps.get("humidity-to-location").unwrap(),
                humidity,
                false,
            );

            (*seed, location)
        })
        .fold(HashMap::new(), |mut acc, v| {
            acc.insert(v.0, v.1);
            acc
        });

    let closest_seed = seeds_with_locations
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;

    *closest_seed
}



//WRONG!!!
//Do it right now...


//The map seems to be missing values
fn parse_map_part_two(map: (Vec<u64>, HashMap<String, Vec<(u64, u64, u64)>>)) -> u64 {
    let seed_list = map.0;
    let maps = map.1;
    //SeedRange::new(range_input[0], range_input[1])

    let parsed_seed_list: Vec<SimpleRange> = seed_list
        .chunks(2)
        .map(|x| SimpleRange::new(x[0], x[1]))
        .collect();

    let parsed_maps = parse_raw_range_maps((&seed_list, &maps));

    println!("Part 2 starting reverse search stage");

    let mut current_seed_group = 0;

    let result = 'lp: loop {
        let locations_with_seeds: Vec<(u64, u64)> = (current_seed_group
            ..current_seed_group + 100_000)
            .into_iter()
            .map(|location| {
                //make a reverse of the above
                let humidity = get_map_out_from_input(
                    parsed_maps.get("humidity-to-location").unwrap(),
                    location,
                    true,
                );
                let temperature = get_map_out_from_input(
                    parsed_maps.get("temperature-to-humidity").unwrap(),
                    humidity,
                    true,
                );
                let light = get_map_out_from_input(
                    parsed_maps.get("light-to-temperature").unwrap(),
                    temperature,
                    true,
                );
                let water =
                    get_map_out_from_input(parsed_maps.get("water-to-light").unwrap(), light, true);
                let fertilizer = get_map_out_from_input(
                    parsed_maps.get("fertilizer-to-water").unwrap(),
                    water,
                    true,
                );
                let soil = get_map_out_from_input(
                    parsed_maps.get("soil-to-fertilizer").unwrap(),
                    fertilizer,
                    true,
                );
                let seed =
                    get_map_out_from_input(parsed_maps.get("seed-to-soil").unwrap(), soil, true);

                (seed, location)
            })
            .collect();

        let mut sorted_seed_locations = locations_with_seeds.clone();
        sorted_seed_locations.sort_by(|a, b| a.1.cmp(&b.1));

        for (seed, location) in locations_with_seeds.iter() {
            for parsed_seed in parsed_seed_list.iter() {
                if parsed_seed.contains(seed.clone()) {
                    break 'lp location.clone();
                }
            }
        }

        current_seed_group += 100_000;
        print!("Hitting {} iterations\n", current_seed_group);
    };

    result
}

fn parse_input(input: &Vec<String>) -> (Vec<u64>, HashMap<String, Vec<(u64, u64, u64)>>) {
    let mut raw_seed_nums: Vec<u64> = Vec::new();

    let mut current_map = "seed".to_string();

    let mut maps: HashMap<String, Vec<(u64, u64, u64)>> = HashMap::new();

    for line in input {
        if line.trim() == "" {
            continue;
        }

        if current_map == "seed" {
            //For part one this is just a list of seeds, for part two this is a seed ranges.
            raw_seed_nums = line
                .split_once(" ")
                .unwrap()
                .1
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            //Skip after setting seed list
            current_map = "seed-done".to_string();
            continue;
        }

        if line.contains(&":".to_string()) {
            current_map = line.split_whitespace().collect::<Vec<&str>>()[0].to_string();
            continue;
        }

        let values_vec = line
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if values_vec.len() != 3 {
            println!("ERROR: {}", line);
            continue;
        }
        let values = (values_vec[0], values_vec[1], values_vec[2]);

        match maps.entry(current_map.clone()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(values);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![values]);
            }
        }
    }

    (raw_seed_nums, maps)
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
