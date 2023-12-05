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

    println!("Seed parsed: {:?}", seed_parsed);

}

fn part_two(input: &Vec<String>) {
    
}



struct RawRangeMap {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

impl RawRangeMap {
    fn new(destination_range_start: u32, source_range_start: u32, range_length: u32) -> RawRangeMap {
        RawRangeMap {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
    //(Source, Destination)
    //If it is not in the map when retrieving it, return that value as the destination.
    fn to_hashmaps(&self) -> HashMap<u32, u32> {
        let mut map = HashMap::new();

        for i in 0..(self.range_length-1) {
            map.insert(self.source_range_start + i, self.destination_range_start + i);
        }

        map
    }
}


fn get_map_out_from_input(map: &HashMap<u32, u32>, input: u32) -> u32 {
    match map.get(&input) {
        Some(out) => {
            *out
        },
        None => {
            input
        },
    }
}


//The map seems to be missing values
fn parse_map_part_one(map: (Vec<u32>, HashMap<String, Vec<(u32, u32, u32)>>)) -> u32 {
    
        let seed_list = map.0;
        let maps = map.1;
    
        let mut parsed_maps: HashMap<String, HashMap<u32, u32>> = HashMap::new();
    
        for (map_name, map_values_vec) in maps {
            let mut map_values: Vec<HashMap<u32, u32>> = Vec::new();
            for map_value in map_values_vec {
                map_values.push(RawRangeMap::new(map_value.0, map_value.1, map_value.2).to_hashmaps());
            }
            parsed_maps.insert(map_name, map_values.iter().fold(HashMap::new(), | mut acc, v | { acc.extend(v.clone()); acc }));
        }
    


        let seeds_with_locations = seed_list.iter().map( | seed | {
    

            //I know it should exist...
            let soil = get_map_out_from_input(parsed_maps.get("seed-to-soil").unwrap(), *seed);
            let fertilizer = get_map_out_from_input(parsed_maps.get("soil-to-fertilizer").unwrap(), soil);
            let water = get_map_out_from_input(parsed_maps.get("fertilizer-to-water").unwrap(), fertilizer);
            let light = get_map_out_from_input(parsed_maps.get("water-to-light").unwrap(), water);
            let temperature = get_map_out_from_input(parsed_maps.get("light-to-temperature").unwrap(), light);
            let humidity = get_map_out_from_input(parsed_maps.get("temperature-to-humidity").unwrap(), temperature);
            let location = get_map_out_from_input(parsed_maps.get("humidity-to-location").unwrap(), humidity);



            println!("Seed: {} soil: {} fertilizer: {} water: {} light: {} temperature: {} humidity: {} location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location);
            




        
            (*seed, location)

        }).fold(HashMap::new(), | mut acc, v| { acc.insert(v.0, v.1); acc });
        //This collected value is the seed back, with the location as its value



        let closest_seed = seeds_with_locations.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;

        println!("Seed list: {:?}", seeds_with_locations);
        println!("soil-to-fertilizer {:?}", parsed_maps.get("soil-to-fertilizer").unwrap());

        println!("fertilizer-to-water {:?}", parsed_maps.get("fertilizer-to-water").unwrap());
        println!("Seeds with locations: {:?}", seeds_with_locations);
        println!("closest_seed: {}", closest_seed);




        *closest_seed

}


fn parse_input(input: &Vec<String>) -> (Vec<u32>, HashMap<String, Vec<(u32, u32, u32)>>) {


    let mut seed_list: Vec<u32> = Vec::new();

    let mut current_map = String::new();


    let mut maps: HashMap<String, Vec<(u32, u32, u32)>> = HashMap::new();


    for line in input {

    
    if line.trim() == "" {
        continue;
    }

    if current_map == ""  {


        seed_list = line.split_once(" ").unwrap().1.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        //Skip after setting seed list
        current_map = "seed-done".to_string();
        continue;
    }




    if line.contains(&":".to_string()) {
        current_map = line.split_whitespace().collect::<Vec<&str>>()[0].to_string();
        continue;
    }



    let values_vec = line.split_whitespace().into_iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
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

    (seed_list, maps)
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
