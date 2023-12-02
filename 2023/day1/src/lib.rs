use std::io::{self, BufRead};
use regex::Regex; // I wish I used regex instead. would be very clean.

pub fn run() {
    let stdin = io::stdin();
    part_one_and_two(&stdin, true);
    //Entry point to the day's code
    


}


fn part_one_and_two(stdin: &io::Stdin, replace_text: bool) {
    //Part 1 of the day's challenge

    let mut sum: u32 = 0;


    for line in stdin.lock().lines() {
        let line: String = line.expect("Could not read line from standard in!");
        
        //each line

        //if it is part 2, replace the text with the numbers


        let replaced_text: String = match replace_text {
            true => {
                line
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
            },
            false => line.clone(),
        };
       



        let mut nums: Vec<u8> = Vec::new();
        for chr in replaced_text.chars() {
            if chr.is_digit(10) {
                nums.push(chr.to_digit(10).unwrap() as u8);
            }
        }

        if nums.len() == 0 {
            println!("No numbers found in line!");
            continue;
        }
        println!("nums: {:?}", nums);
        println!("them added: {}", (nums[0] as u32) * 10 + (nums[nums.len() - 1] as u32));
        sum += (nums[0] as u32) * 10 + (nums[nums.len() - 1] as u32);


    }
    println!("Sum: {}", sum);
    
}