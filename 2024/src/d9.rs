fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}
pub fn parse_input(raw_input: &str) -> Vec<u8> {
    strip_trailing_newline(raw_input.trim())
        .chars()
        .map(|f| f.to_digit(10).expect("Number") as u8)
        .collect()
}

pub fn solve1(raw_input: &str) -> String {
    let mut input = parse_input(raw_input);

    let mut sum = 0;

    let mut final_pos = 0;

    'outer: for id in 0..input.len() {
        if id % 2 == 0 {
            for _ in 0..input[id] {
                sum += final_pos * (id / 2);
                final_pos += 1;
            }
        } else {
            for _ in 0..input[id] {
                let Some((id, _)) = input
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(iid, val)| iid % 2 == 0 && **val != 0 && *iid > id)
                else {
                    break 'outer;
                };
                input[id] -= 1;
                sum += final_pos * (id / 2);
                final_pos += 1;
            }
        }
    }
    return sum.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let mut input = parse_input(raw_input);

    let mut sum = 0;

    let mut final_pos = 0;

    for id in 0..input.len() {
        if id % 2 == 0 {
            for _ in 0..input[id] {
                sum += final_pos * (id / 2);
                final_pos += 1;
            }
        } else {
            let mut used_count = 0;
            loop {
                if let Some((id, val)) = input.iter().enumerate().rev().find(|(iid, val)| {
                    iid % 2 == 0 && **val != 0 && **val <= (input[id] - used_count) && *iid > id
                }) {
                    for _ in 0..*val {
                        sum += final_pos * (id / 2);
                        final_pos += 1;
                        used_count += 1;
                    }
                    final_pos += (input[id] - val) as usize;
                    input[id] = 0;
                } else {
                    final_pos += (input[id] - used_count) as usize;
                    break;
                }
            }
        }
    }
    return sum.to_string();
}
