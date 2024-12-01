use common::{get_input, reverse_iter};

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run() {
    part_one(get_input());

    part_two(get_input());
    //Entry point to the day's code
}

fn part_one(lines_in: Vec<String>) {
    let sum_part_one: u32 = lines_in
        .iter()
        .map(|line| {
            (match line.chars().find(|x| x.is_digit(10)) {
                Some(value) => value.to_digit(10).unwrap(),
                None => 0,
            }) + (match line.chars().rev().find(|x: &char| x.is_digit(10)) {
                Some(value) => value.to_digit(10).unwrap(),
                None => 0,
            })
        })
        .sum();
}

fn find_first_occurence<const COUNT: usize>(
    line: &String,
    list: [&str; COUNT],
    reverse: bool,
) -> u32 {
    for (i, c) in reverse_iter(line.chars(), reverse).enumerate() {
        if let Some(num) = c.to_digit(10) {
            return num;
        } else if let Some(str_value) = list.iter().enumerate().find(|x| {
            reverse_iter(line.chars(), reverse)
                .skip(i)
                .take(x.1.len())
                .eq(reverse_iter(x.1.chars(), reverse))
        }) {
            return str_value.0 as u32 + 1;
        }
    }

    return 0;
}

//Mostly 2 now
fn part_two(lines_in: Vec<String>) -> u32 {
    let value = lines_in
        .iter()
        .map(|line| {
            find_first_occurence(line, NUMBERS, false) * 10
                + find_first_occurence(line, NUMBERS, true)
        })
        .sum::<u32>();

    println!("Part two: {value}");
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_and_two() {
        let test_vec_small = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
            // "oneightwoneight".to_string()
        ];

        let new_other_test = include_str!("./part_two_input.txt")
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let time = std::time::Instant::now();

        for _ in 0..1000 {
            part_two(new_other_test.clone());
        }
        let output = part_two(new_other_test);
        println!("{:?}", time.elapsed());
        assert_eq!(output, 55614);

        //Ten seconds to do 1000 iterations of part two on my machine (with my given input) without printing, seems okay.
    }
}
