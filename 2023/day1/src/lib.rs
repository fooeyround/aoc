use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead}, ops::Index, vec, path::Iter,
};
use either::{self, Either};



fn reverse_itr<'a, Container: DoubleEndedIterator<Item=T> + , T>(into_itr: Container, reverse: bool) -> Either<std::iter::Rev<<Container as IntoIterator>::IntoIter>, <Container as IntoIterator>::IntoIter>  {
    let mut itr = into_itr.into_iter();
    if reverse { 
        Either::Left(itr.rev()) 
    }
    else { 
        Either::Right(itr) 
    }
}


pub fn run() {
    let stdin = io::stdin();
    let lines_in: Vec<String> = stdin
        .lock()
        .lines()
        .map(|v| match v {
            Ok(val) => val,
            Err(e) => {
                println!("Error reading line from standard in! {}", e);
                String::from("")
            }
        })
        .collect();



    part_one(lines_in.clone());
    part_two(lines_in);
    //Entry point to the day's code
}



fn part_one(lines_in: Vec<String>) {
    let sum_part_one: u32 = lines_in.iter().map( | line | {

        (match line.chars().find(|x| {x.is_digit(10)}) {
            Some(value) => {
                value.to_digit(10).unwrap()
            }
            None => { 0 },
        }) + (match line.chars().rev().find(|x: &char| { x.is_digit(10)} ) {
            Some(value) => {
                value.to_digit(10).unwrap()
            },
            None => { 0 },
        })
    }).sum();

}






fn find_first_occurences(line: &String, reverse: bool) -> u32 {
    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
   


   

    for (i, c) in reverse_itr(line.chars(), reverse).enumerate() {
        if let Some(num) = c.to_digit(10) {
            println!("Found number {} at index {}", num, i);
            return num;
        } else if let Some(str_value) = NUMBERS.iter().enumerate()
        .find(|x| { reverse_itr(line.chars(), reverse)
            .skip(i).take(x.1.len()).eq(reverse_itr(x.1.chars(), reverse)) }) {
            println!("Found string: {} & {}", str_value.0, str_value.1);
            return str_value.0 as u32 + 1;
        }
    }


    return 0;

}


//Mostly 2 now
fn part_two(lines_in: Vec<String>) {



    println!("Part two: {}", lines_in.iter().map(|line| {
        println!("Line: {}", line);

        println!("First occurence: {}", find_first_occurences(line, false));
        println!("Second occurence: {}", find_first_occurences(line, true));

        find_first_occurences(line, false) * 10 
        +
        find_first_occurences(line, true)
    }).sum::<u32>());


}

#[test]
fn test_part_one_and_two() {
    let vec_of = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
        // "oneightwoneight".to_string()

    ];
    // part_one_and_two(vec_of, true);
    //assert_eq!(part_one_and_two(&stdin, false), 0);

    part_two(vec_of);
}
