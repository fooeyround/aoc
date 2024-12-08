use itertools::Itertools;

pub fn solve1(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    let count = input
        .iter()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|f| f.parse::<u32>().expect("An Unsigned Integer"))
                .collect::<Vec<u32>>();

            let mut last = nums[0];
            let increasing = nums[1] > nums[0];
            for num in nums[1..].iter() {
                let match_dir = if increasing { *num > last } else { *num < last };

                if !(match_dir && num.abs_diff(last) <= 3) {
                    return false;
                }
                last = *num;
            }

            return true;
        })
        .count();

    return count.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    return input
        .iter()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|f| f.parse::<u32>().expect("An Unsigned Integer"))
                .collect::<Vec<u32>>();

            for incr in [true, false].iter() {
                for iiddxx in 0..nums.len() {
                    let correct_count = nums
                        .iter()
                        .enumerate()
                        .filter(|&(idx, _)| iiddxx != idx)
                        .tuple_windows()
                        .filter(|((_,a), (_,b))| if *incr { a < b } else { a > b } && a.abs_diff(**b) <=3).count();

                    if correct_count >= nums.len()-2 {
                        return true;
                    }
                }
            }
            return false;

        })
        .count()
        .to_string();
}
