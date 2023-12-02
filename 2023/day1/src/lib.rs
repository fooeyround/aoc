
use std::{io::{self, BufRead}, ops::Index, collections::{hash_map, HashMap}};
use regex::Regex; // I wish I used regex instead. would be very clean.

pub fn run() {
    let stdin = io::stdin();
    let lines_in: Vec<String> = stdin.lock().lines().map( |v| { 
        match v {
            Ok(val) => {
                val
            },
            Err(e) => {
                println!("Error reading line from standard in! {}", e);
                String::from("")
            },
        }
    }).collect();
    part_one_and_two(lines_in, true);
    //Entry point to the day's code
    


}


//Mostly 2 now
fn part_one_and_two(lines_in: Vec<String>, replace_text: bool) {
    //Part 1 of the day's challenge

    let mut sum: u32 = 0;


    //for part 2
    let num_types: HashMap<String, String> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ].iter().cloned().map( | (k, v) | { (k.to_string(), v.to_string()) }).collect();


    for line in lines_in {

        println!("Line: {}", line);
        
        //each line

        //if it is part 2, replace the text with the numbers


        let replaced_text: String = match replace_text {
            true => {



                let mut line_vec: Vec<char> = line.clone().chars().collect();

            
                'outer: for i in 0..line_vec.len() {
                    if i >= line_vec.len() || line_vec[i].is_digit(10) {
                        continue;
                    }

                    'inner: for num_type in num_types.iter() {
   
                        let str_as_char: Vec<char> = num_type.0.chars().collect();
            
                        if line_vec[i..].starts_with(str_as_char.as_slice()) {
                            for itr in i..(num_type.0.len() + i) {
                                if itr < line_vec.len() {
                                    line_vec[itr] = ' ';
                                    
                                }
                            }
                           
        
                            //Was the ' ' char.
                            line_vec.remove(i);
                            line_vec.insert(i, num_type.1.chars().last().unwrap());
                            continue 'outer;
                        }
                    }

                }

              
                line_vec.iter().collect::<String>()
                

            },
            false => line.clone(),
        };
       





        let mut nums: Vec<u8> = Vec::new();
        for chr in replaced_text.chars() {
            if chr.is_digit(10) {
                nums.push(chr.to_digit(10).unwrap() as u8);
            }
        }

        println!("Numbers: {:?}", nums);

        if nums.len() == 0 {
            println!("No numbers found in line!");
            continue;
        }
        let line_sum = (nums[0] as u32) * 10 + (nums[nums.len() - 1] as u32);
        sum += line_sum;

        println!("Line sum: {}", line_sum);


    }
    println!("Sum: {}", sum);
    
}


#[test]
fn test_part_one_and_two() {
    let vec_of = vec![
        "one23nineight".to_string(),
        "twothree46f".to_string(),
        "threetwo4forme".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "sevenine".to_string(),
        "eight".to_string(),
        "eightnine".to_string(),
        "nineight".to_string(),
    ];
    part_one_and_two(vec_of, true);
    //assert_eq!(part_one_and_two(&stdin, false), 0);
}