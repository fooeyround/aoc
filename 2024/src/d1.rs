pub fn solve1(input: &Vec<String>) -> String {
    let mut lists = (Vec::new(), Vec::new());

    for line in input {
        let Some(bb) = line.trim().split_once(" ") else {
            continue;
        };
        lists.0.push(bb.0.trim().parse::<u32>().expect("Number"));
        lists.1.push(bb.1.trim().parse::<u32>().expect("Number"));
    }

    lists.0.sort();
    lists.1.sort();

    let mut count = 0;
    for i in lists.0.iter().zip(lists.1.iter()) {
        count += (i.0).abs_diff(*i.1);
    }
    return count.to_string();
}
pub fn solve2(input: &Vec<String>) -> String {
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
