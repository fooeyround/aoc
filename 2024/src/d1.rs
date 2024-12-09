pub fn solve1(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input
        .iter()
        .filter_map(|line| line.trim().split_once(" "))
        .map(|(a, b)| {
            (
                a.trim().parse::<u32>().expect("Number"),
                b.trim().parse::<u32>().expect("Number"),
            )
        })
        .collect();

    a.sort();
    b.sort();

    let mut count = 0;
    for (i, j) in a.iter().zip(b.iter()) {
        count += (i).abs_diff(*j);
    }
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

    let mut lists = (Vec::new(), Vec::new());

    for line in input {
        let Some(bb) = line.trim().split_once(" ") else {
            continue;
        };
        lists.0.push(bb.0.trim().parse::<u32>().expect("Number"));
        lists.1.push(bb.1.trim().parse::<u32>().expect("Number"));
    }

    let sum: u32 = lists
        .0
        .iter()
        .map(|f| f * lists.1.iter().filter(|a| (**a == *f)).count() as u32)
        .sum();

    return sum.to_string();
}
