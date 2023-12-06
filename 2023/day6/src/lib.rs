use std::collections::HashMap;

use common::get_input;

pub fn run() {
    
let input = get_input();

println!("Day 6");
part_one(&input);

part_two(&input);

}


fn parse_part_one(input: &Vec<String>) -> HashMap<u32, u32> {



    let mut vec_map: HashMap<String, Vec<u32>> = HashMap::new();

    for line in input {
        let values = line.split_once(" ").unwrap();
        let mut name_chars = values.0.chars();
        name_chars.next_back();
        vec_map.insert(name_chars.collect(), values.1.split(" ").filter_map(|x| {
            if !x.trim().is_empty() {
                Some(x.trim().parse::<u32>().unwrap())
            } else {
                None
            }
        }).collect());
    }


    let times = vec_map.get("Time").unwrap();
    let distance = vec_map.get("Distance").unwrap();

    let mut time_distance: HashMap<u32, u32> = times.iter().zip(distance.iter()).map(|(x, y)| (*x, *y)).collect();
    let mut vec_map: HashMap<String, Vec<u32>> = HashMap::new();


    return time_distance;





}


fn parse_part_two(input: &Vec<String>) -> (u64, u64) {



    let time: u64 = input.get(0).unwrap().split_once(" ").unwrap().1.trim().replace(" ", "").parse().unwrap();
    let distance: u64 = input.get(1).unwrap().split_once(" ").unwrap().1.trim().replace(" ", "").parse().unwrap();












    return (time, distance);





}


fn how_far(charge_time: u32, time_left: u32) -> u32 {

    //the charge time is the speed

    return charge_time * time_left;

}


fn part_one(input: &Vec<String>) {

    let parsed = parse_part_one(input);

    let answer = parsed.iter().map( | val | {
        let mut distances: Vec<u32> = Vec::new();

        for charge_time in 0..*val.0 {
            let time_left = val.0 - charge_time;
            let this_distance = charge_time * time_left;
            if this_distance > *val.1 {
                distances.push(charge_time);
            }
            //run function on charge time to see how far it gets within the time left
        }
        distances
    }).fold(0, | acc, val| {
        if acc == 0 {
            val.len()
        } else {
            acc * val.len()
        }
    });

    // }


    println!("Part One: {}", answer);



}


fn part_two(input: &Vec<String>) {

    let parsed = parse_part_two(input);

        let mut distances: Vec<u64> = Vec::new();

        println!("{:?} | {:?}", parsed.0, parsed.1);
        for charge_time in 0..parsed.0 {
            let time_left = parsed.0 - charge_time;
            let this_distance = charge_time * time_left;
            if this_distance >= parsed.1 {
                distances.push(charge_time);
            }
            //run function on charge time to see how far it gets within the time left
        }
        
    

    // }


    println!("Part 2: {:?}", distances.len());



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
