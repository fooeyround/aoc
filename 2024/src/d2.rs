pub fn solve1(input: &Vec<String>) -> String {
    let count = 0;

    let b: Vec<_> = input
        .iter()
        .map(|f| {
            let nums = f
                .split_whitespace()
                .map(|f| f.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let diffs = nums
                .windows(2)
                .map(|f| f[1] as i64 - f[0] as i64)
                .collect::<Vec<i64>>();

            let count_of_out_of_range = diffs.iter().filter(|f| (*f).abs() > 3).count();

            let increasing_count = diffs.iter().filter(|f| **f > 0).count();
            let decreasing_count = diffs.iter().filter(|f| **f < 0).count();
        })
        .collect();

    return String::new();
}
pub fn solve2(input: &Vec<String>) -> String {
    return String::new();
}
