fn comes_after(insertee: u32, inserted: u32, rules: Vec<(u32, u32)>) -> bool {
    let mut curr = insertee;

    for _ in 0..100000 {
        if let Some(val) = rules.iter().find(|(a, b)| *b == curr) {
            curr = val.0;
        }
        if (curr == inserted) {
            return true;
        }
    }

    return false;
}

pub fn solve1(raw_input: &str) -> String {
    let (raw_rules, raw_manuals) = raw_input.split_once("\n\n").expect("Double newline");

    let rules: Vec<(u32, u32)> = raw_rules
        .split("\n")
        .map(|f| f.split_once("|").expect("A Pipe"))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut manuals: Vec<Vec<u32>> = raw_manuals
        .split("\n")
        .map(|f| {
            f.split(",")
                .filter(|f| !f.trim().is_empty())
                .map(|f| f.trim().parse::<u32>().expect("Number"))
                .collect::<Vec<u32>>()
        })
        .filter(|f| !f.is_empty())
        .collect();

    for manual in &mut manuals {
        for idx in 1..manual.len() {
            manual.swap(idx - 1, idx);
        }
    }

    return String::new();
}
pub fn solve2(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };
    return String::new();
}
