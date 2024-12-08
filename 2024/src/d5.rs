use std::cmp::Ordering;

fn comes_before(a: u32, b: u32, rules: &Vec<(u32, u32)>) -> bool {
    return rules.iter().find(|(aa, bb)| *aa == a && *bb == b).is_some();
}

fn get_rule_manuals(raw_input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (raw_rules, raw_manuals) = raw_input.split_once("\n\n").expect("Double newline");
    let mut rules: Vec<(u32, u32)> = raw_rules
        .split("\n")
        .map(|f| f.split_once("|").expect("A Pipe"))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    rules.sort_by(|a, b| (a.0).cmp(&b.0));

    let manuals: Vec<Vec<u32>> = raw_manuals
        .split("\n")
        .map(|f| {
            f.split(",")
                .filter(|f| !f.trim().is_empty())
                .map(|f| f.trim().parse::<u32>().expect("Number"))
                .collect::<Vec<u32>>()
        })
        .filter(|f| !f.is_empty())
        .collect();
    return (rules, manuals);
}

pub fn solve1(raw_input: &str) -> String {
    let (rules, manuals) = get_rule_manuals(raw_input);

    let correctly_ordered_manuals: Vec<&Vec<u32>> = manuals
        .iter()
        .filter(|manual| {
            manual
                .windows(2)
                .all(|ab| comes_before(ab[0], ab[1], &rules))
        })
        .collect();

    let sum: u32 = correctly_ordered_manuals
        .iter()
        .map(|f| f[(f.len() - 1) / 2])
        .sum();

    return sum.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let (rules, mut manuals) = get_rule_manuals(raw_input);

    let mut incorrect_manuals: Vec<&mut Vec<u32>> = manuals
        .iter_mut()
        .filter(|manual| {
            !manual
                .windows(2)
                .all(|ab| comes_before(ab[0], ab[1], &rules))
        })
        .collect();

    for manual in &mut incorrect_manuals {
        manual.sort_by(|a, b| {
            if comes_before(*a, *b, &rules) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    return incorrect_manuals
        .iter()
        .map(|f| f[(f.len() - 1) / 2])
        .sum::<u32>()
        .to_string();
}
