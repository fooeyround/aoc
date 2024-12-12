use itertools::{repeat_n, Itertools};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn solve1(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    let items = input
        .iter()
        .map(|f| f.split_once(":").expect("Colon"))
        .map(|(a, b)| {
            (
                a.trim().parse::<u64>().expect("Number"),
                b.trim()
                    .split_whitespace()
                    .map(|f| f.parse::<u64>().expect("Number"))
                    .collect::<Vec<u64>>(),
            )
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let sum = items
        .par_iter()
        .filter_map(|(combied, expr_parts)| {
            for perm in repeat_n(0..2, expr_parts.len()).multi_cartesian_product() {
                let mut tmp_sum = 0;
                for (i, val) in expr_parts.iter().enumerate() {
                    match perm[i] {
                        0 => tmp_sum *= val,
                        1 => tmp_sum += val,
                        _ => {}
                    }
                }

                if tmp_sum == *combied {
                    return Some(combied);
                }
            }
            return None;
        })
        .sum::<u64>();

    return sum.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    let items = input
        .iter()
        .map(|f| f.split_once(":").expect("Colon"))
        .map(|(a, b)| {
            (
                a.trim().parse::<u64>().expect("Number"),
                b.trim()
                    .split_whitespace()
                    .map(|f| f.parse::<u64>().expect("Number"))
                    .collect::<Vec<u64>>(),
            )
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let sum = items
        .par_iter()
        .filter_map(|(combied, expr_parts)| {
            for perm in repeat_n(0..3, expr_parts.len()).multi_cartesian_product() {
                let mut tmp_sum = 0;
                for (i, val) in expr_parts.iter().enumerate() {
                    match perm[i] {
                        0 => tmp_sum *= val,
                        1 => tmp_sum += val,
                        2 => {
                            tmp_sum = tmp_sum
                                * 10u64.pow((*val as f64 + 1.0).log10().ceil() as u32)
                                + val;
                        }
                        _ => {}
                    }
                }

                if tmp_sum == *combied {
                    return Some(combied);
                }
            }
            return None;
        })
        .sum::<u64>();

    return sum.to_string();
}
