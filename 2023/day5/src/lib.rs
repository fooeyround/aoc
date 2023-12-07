use std::{
    collections::{btree_set::Intersection, HashMap},
    iter,
    ops::Add,
};

use common::get_input;
use itertools::Itertools;

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



        if self.start + self.length < other_range.start
            || other_range.start + other_range.length < self.start
        {
            return None;
        }
        let new_start = self.start.max(other_range.start);
        return Some(SimpleRange::new(
            new_start,
            (self.length + self.start).min(other_range.length + other_range.start) - new_start,
        ));
    }



    fn move_over_range_map(&self, other_ranges: &Vec<RangeMap>, reverse: bool) -> Vec<SimpleRange> {
        let mut out: Vec<SimpleRange> = Vec::new();
        let mut to_convert: Vec<SimpleRange> = Vec::new();

        to_convert.push(self.clone());

        for range_map in other_ranges {

            let relative_s_start = if reverse {
                range_map.destination_range_start
            } else {
                range_map.source_range_start
            };
            let relative_d_start = if !reverse {
                range_map.destination_range_start
            } else {
                range_map.source_range_start
            };


            





            let range_shift = relative_d_start as i64 - relative_s_start as i64;


            //how do I stop range_shift from underflowing?

            let output_source_range = SimpleRange::new(relative_s_start, range_map.length);

            to_convert = to_convert.iter().map(|x| {
                //do map over range_map then apply this to out!

                let mut new_to_convert: Vec<SimpleRange> = Vec::new();

                if let Some(intersection) = x.intersection(&output_source_range) {
                    let subtracted = x.subtract(&intersection);


                    
                    if (range_shift + (intersection.start as i64)) < 0.0 as i64 {
                        return new_to_convert;
                    }



                    println!("range_shift: {} intersection: {:?}", range_shift, intersection);
                    // let shifted_range = intersection + range_shift;
    
                    if let Some(left) = subtracted.0 {
                        new_to_convert.push(left);
                    }
                    if let Some(right) = subtracted.1 {
                        new_to_convert.push(right);
                    }
    
                    // out.push(shifted_range);

            
                }

                return new_to_convert;

            }).flatten().collect();
        }

        // If the value is not found, it is an identity.
        out.extend(to_convert);

        out
    }

    #[inline]
    fn get_end(&self) -> u64 {
        return self.start + self.length;
    }

    fn subtract(&self, other: &Self) -> (Option<SimpleRange>, Option<SimpleRange>) {
        //cut the range from the front half
        let optional_intersection = self.intersection(&other);
        if let Some(intersection) = optional_intersection {
            if self.start == intersection.start && self.length == intersection.length {
                //-----
                //-----
                return (None, None);
            }

            if self.start == intersection.start {
                // ------------
                // ------
                return (
                    None,
                    Some(SimpleRange::new(
                        intersection.get_end(),
                        self.length - intersection.length,
                    )),
                );
            }

            if intersection.get_end() == self.get_end() {
                //----------
                //      ----
                return (Some(SimpleRange::new(self.start, intersection.start)), None);
            }

            if intersection.start > self.start && intersection.get_end() < self.get_end() {
                //----------------
                //      ----
                return (
                    Some(SimpleRange::new(self.start, intersection.start)),
                    Some(SimpleRange::new(
                        intersection.get_end(),
                        self.get_end() - intersection.get_end(),
                    )),
                );
            }
        }

        //If there is no intersection, return the original range, it could not have been modified.
        return (Some(self.clone()), None);
    }
}

impl std::ops::Add<u64> for SimpleRange {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        SimpleRange::new(self.start + rhs, self.length)
    }
}

impl std::ops::Add<i64> for SimpleRange {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        //This is mildly unsafe, but I think it is fine.
        SimpleRange::new(
            //The MAX call will limit the number, but it already does that..
            //The answer should be within u32, so it should be fine.
            (self.start as i64 + rhs).clamp(0, i64::MAX) as u64,
            self.length,
        )
    }
}

#[derive(Debug, Clone)]
struct RangeMap {
    destination_range_start: u64,
    source_range_start: u64,
    length: u64,
}

impl RangeMap {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> RangeMap {
        RangeMap {
            destination_range_start,
            source_range_start,
            length: range_length,
        }
    }


}

impl std::ops::Sub for RangeMap {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        RangeMap::new(
            self.destination_range_start - other.destination_range_start,
            self.source_range_start - other.source_range_start,
            self.length - other.length,
        )
    }
}

impl std::ops::Add for RangeMap {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        RangeMap::new(
            self.destination_range_start
                .min(other.destination_range_start),
            self.source_range_start.min(other.source_range_start),
            self.length.max(other.length),
        )
    }
}

///Returns a new hashmap with the keys and values of the one passed swapped.

fn get_map_out_from_input_part_one(maps: &Vec<RangeMap>, input: u64, reverse: bool) -> u64 {
    //if reverse call the helper or switch case?

    for raw_range_map in maps {
        match reverse {
            false => {
                if raw_range_map.source_range_start <= input
                    && input < raw_range_map.source_range_start + raw_range_map.length
                {
                    return input - raw_range_map.source_range_start
                        + raw_range_map.destination_range_start;
                }
            }

            //Handle reversed case
            true => {
                if raw_range_map.destination_range_start <= input
                    && input < raw_range_map.destination_range_start + raw_range_map.length
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
) -> HashMap<String, Vec<RangeMap>> {
    map.1
        .iter()
        .map(|map| {
            let mut map_values: Vec<RangeMap> = Vec::new();
            for map_value in map.1 {
                map_values.push(RangeMap::new(map_value.0, map_value.1, map_value.2));
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
                get_map_out_from_input_part_one(&parsed_maps.get("seed-to-soil").unwrap(), *seed, false);
            let fertilizer =
                get_map_out_from_input_part_one(parsed_maps.get("soil-to-fertilizer").unwrap(), soil, false);
            let water = get_map_out_from_input_part_one(
                parsed_maps.get("fertilizer-to-water").unwrap(),
                fertilizer,
                false,
            );
            let light =
                get_map_out_from_input_part_one(parsed_maps.get("water-to-light").unwrap(), water, false);
            let temperature = get_map_out_from_input_part_one(
                parsed_maps.get("light-to-temperature").unwrap(),
                light,
                false,
            );
            let humidity = get_map_out_from_input_part_one(
                parsed_maps.get("temperature-to-humidity").unwrap(),
                temperature,
                false,
            );
            let location = get_map_out_from_input_part_one(
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



  


    let valid_location_ranges = parsed_seed_list
        .iter()
        .map(|seed_range| {


            seed_range.move_over_range_map(
                &parsed_maps.get("seed-to-soil").unwrap(),
                false,
            )
                .iter()
                .map(|x| {
                    x.move_over_range_map(&parsed_maps.get("soil-to-fertilizer").unwrap(), false)
                })
                .flatten()


                .map(|x| {
                    x.move_over_range_map(&parsed_maps.get("fertilizer-to-water").unwrap(), false)
                })
                .flatten()


                .map(|x| {
                    x.move_over_range_map(&parsed_maps.get("water-to-light").unwrap(), false)
                })
                .flatten()


                .map(|x| {
                    x.move_over_range_map(&parsed_maps.get("light-to-temperature").unwrap(), false)
                })
                .flatten()


                .map(|x| {
                    x.move_over_range_map(&parsed_maps.get("temperature-to-humidity").unwrap(), false)
                })
                .flatten()


                .map(|x| {
                    x.move_over_range_map(&parsed_maps.get("humidity-to-location").unwrap(), false)
                })
                .flatten()
                .collect::<Vec<SimpleRange>>()
        })
        .flatten()
        .collect::<Vec<SimpleRange>>();




    let sorted_valid_locations = valid_location_ranges
        .iter()
        .sorted_by(|a, b| a.start.cmp(&b.start))
        .collect::<Vec<&SimpleRange>>();






        sorted_valid_locations.iter().for_each(|x| {
            println!("{:?}", x);
        });
    
    sorted_valid_locations.get(0).unwrap_or(&&SimpleRange::new(0, 0)).start
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
