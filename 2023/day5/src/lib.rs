use std::collections::HashMap;

use common::get_input;

pub fn run() {
    println!("Day 5");

    let input = get_input();
    part_one(&input);
    part_two(&input);
}




fn part_one(input: &Vec<String>) {
    

    let parsed = parse_input(input);

    let seed_parsed = parse_map_part_one(parsed);

    println!("Closest Seed location: {:?}", seed_parsed);

}

fn part_two(input: &Vec<String>) {
    
}


#[derive(Debug, Clone)]
struct RawRangeMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl RawRangeMap {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> RawRangeMap {
        RawRangeMap {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
    //(Source, Destination)
    //If it is not in the map when retrieving it, return that value as the destination.
    fn to_hashmaps(&self) -> HashMap<u64, u64> {
        let mut map = HashMap::new();

        // println!("source range start: {}, destination range start: {}, range length: {}", self.source_range_start, self.destination_range_start, self.range_length);
        // print!("----\n");
        for i in 0..(self.range_length) {
            // println!("i: {}; {:?}", i, 0..(self.range_length));
            map.insert(self.source_range_start + i, self.destination_range_start + i);
        }

        map
    }
}


///Returns a new hashmap with the keys and values of the one passed swapped.




fn get_map_out_from_input(maps: Vec<RawRangeMap>, input: u64, reverse: bool) -> u64 {

    //if reverse call the helper or switch case?

    for raw_range_map in maps {

        let matched_value = match reverse {

            false => {
                if raw_range_map.source_range_start <= input && input < raw_range_map.source_range_start + raw_range_map.range_length {
                    return input - raw_range_map.source_range_start + raw_range_map.destination_range_start;
                }
            },




            //Handle reversed case
            true => {
                if raw_range_map.destination_range_start <= input && input < raw_range_map.destination_range_start + raw_range_map.range_length {
                   return input - raw_range_map.destination_range_start + raw_range_map.source_range_start;
                }
            }
        };


    }




    input
}


//The map seems to be missing values
fn parse_map_part_one(map: (Vec<u64>, HashMap<String, Vec<(u64, u64, u64)>>)) -> u64 {
    
        let seed_list = map.0;
        let maps = map.1;
    
    

        let parsed_maps: HashMap<String, Vec<RawRangeMap>> = maps.iter().map(| map,  | {

            let mut map_values: Vec<RawRangeMap> = Vec::new();
            for map_value in map.1 {
                map_values.push(RawRangeMap::new(map_value.0, map_value.1, map_value.2));
            }
            (map.0.clone(), map_values)
        }).collect();





        let seeds_with_locations = seed_list.iter().map( | seed | {
    

            //I know it should exist... this still is a pain of unwrap and rigid code.
            let soil = get_map_out_from_input(parsed_maps.get("seed-to-soil").unwrap().clone(), *seed, false);
            let fertilizer = get_map_out_from_input(parsed_maps.get("soil-to-fertilizer").unwrap().clone(), soil, false);
            let water = get_map_out_from_input(parsed_maps.get("fertilizer-to-water").unwrap().clone(), fertilizer, false);
            let light = get_map_out_from_input(parsed_maps.get("water-to-light").unwrap().clone(), water, false);
            let temperature = get_map_out_from_input(parsed_maps.get("light-to-temperature").unwrap().clone(), light, false);
            let humidity = get_map_out_from_input(parsed_maps.get("temperature-to-humidity").unwrap().clone(), temperature, false);
            let location = get_map_out_from_input(parsed_maps.get("humidity-to-location").unwrap().clone(), humidity, false);



            //println!("Seed: {} soil: {} fertilizer: {} water: {} light: {} temperature: {} humidity: {} location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location);
            




        
            (*seed, location)

        }).fold(HashMap::new(), | mut acc, v| { acc.insert(v.0, v.1); acc });
        //This collected value is the seed back, with the location as its value



        let closest_seed = seeds_with_locations.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;

        // println!("Seed list: {:?}", seeds_with_locations);
        // println!("soil-to-fertilizer {:?}", parsed_maps.get("soil-to-fertilizer").unwrap());

        // println!("fertilizer-to-water {:?}", parsed_maps.get("fertilizer-to-water").unwrap());
        // println!("Seeds with locations: {:?}", seeds_with_locations);
        // println!("closest_seed: {}", closest_seed);




        *closest_seed

}


fn parse_input(input: &Vec<String>) -> (Vec<u64>, HashMap<String, Vec<(u64, u64, u64)>>) {


    let mut raw_seed_nums: Vec<u64> = Vec::new();

    let mut current_map = "seed".to_string();


    let mut maps: HashMap<String, Vec<(u64, u64, u64)>> = HashMap::new();


    for line in input {

    
    if line.trim() == "" {
        continue;
    }

    if current_map == "seed"  {


        //For part one this is just a list of seeds, for part two this is a seed ranges.
        raw_seed_nums = line.split_once(" ").unwrap().1.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        //Skip after setting seed list
        current_map = "seed-done".to_string();
        continue;
    }




    if line.contains(&":".to_string()) {
        current_map = line.split_whitespace().collect::<Vec<&str>>()[0].to_string();
        continue;
    }



    let values_vec = line.split_whitespace().into_iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    if values_vec.len() != 3 {
        println!("ERROR: {}", line);
        continue;
    }
    let values = (values_vec[0], values_vec[1], values_vec[2]);




    match maps.entry(current_map.clone()) {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().push(values);
        },
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![values]);
        },
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
