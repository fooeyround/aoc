use itertools::Itertools;

pub fn solve1(input: &Vec<String>) -> String {
    let count = input
        .iter()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|f| f.parse::<u32>().expect("An Unsigned Integer"))
                .collect::<Vec<u32>>();

            let mut inititialized = 0;
            let mut last = 0;
            let mut increasing = false;
            for num in nums {
                if inititialized == 0 {
                    last = num;
                    inititialized = 1;
                    continue;
                }
                if inititialized == 1 {
                    increasing = num > last;
                    inititialized = 2;
                }

                let match_dir = if increasing { num > last } else { num < last };

                if !(match_dir && num.abs_diff(last) <= 3) {
                    return false;
                }
                last = num;
            }

            return true;
        })
        .count();

    return count.to_string();
}
pub fn solve2(input: &Vec<String>) -> String {
    return input
        .iter()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|f| f.parse::<u32>().expect("An Unsigned Integer"))
                .collect::<Vec<u32>>();
            let diffs: Vec<i64> = nums
                .windows(2)
                .map(|nums| (nums[1] as i64 - nums[0] as i64))
                .collect();

            for incr in [true, false].iter() {
                for iiddxx in 0..=nums.len() {
                    let correct_count = nums
                        .iter()
                        .enumerate()
                        .filter(|&(idx, num)| iiddxx != nums.len() && iiddxx != idx)
                        .tuple_windows()
                        .filter(|((idxa,a), (idxb,b))| if *incr { a < b } else { a > b } && a.abs_diff(**b) <=3).count();

                    if correct_count > 0 {
                        println!("idx {iiddxx} , correct_count: {correct_count}");
                    }
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
