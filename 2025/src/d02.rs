fn digit_count(value: u64) -> u32 {
    return 1 + value.checked_ilog10().unwrap_or(0);
}

pub fn p1(input: &str) -> String {
    let mut total: u64 = 0;

    for raw_range in input.trim().split(',') {
        let range = raw_range
            .split_once('-')
            .map(|(l, h)| {
                (
                    l.trim().parse::<u64>().unwrap(),
                    h.trim().parse::<u64>().unwrap(),
                )
            })
            .unwrap();

        let max_k = digit_count(range.1) / 2;

        for k in 1..=max_k {
            let pow10_k = 10u64.pow(k);
            let factor = pow10_k + 1;

            let lower_min = 10u64.pow(k - 1);
            let lower_max = pow10_k - 1;

            let start = ((range.0 + factor - 1) / factor).max(lower_min);
            let end = (range.1 / factor).min(lower_max);

            if start <= end {
                let count = end - start + 1;
                let sum_lower = (start + end) * count / 2;
                total += factor * sum_lower;
            }
        }
    }

    total.to_string()
}

pub fn p2(input: &str) -> String {
    return String::new();
}
