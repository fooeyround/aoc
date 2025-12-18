pub fn p1(input: &str) -> String {
    let mut count = 0;
    let mut rot = 50;

    for line in input.split("\n").filter(|l| !l.trim().is_empty()) {
        let left = line.starts_with('L');
        let mult = if left { -1 } else { 1 };
        let rel_rot = i32::from_str_radix(&line[1..], 10).expect("int") * mult;
        rot = (rot + rel_rot).rem_euclid(100);
        if rot == 0 {
            count += 1;
        }
    }

    return count.to_string();
}
pub fn p2(input: &str) -> String {
    let mut count = 0;
    let mut rot = 50;

    for line in input.split("\n").filter(|l| !l.trim().is_empty()) {
        let left = line.starts_with('L');
        let mult = if left { -1 } else { 1 };
        let rel = i32::from_str_radix(&line[1..], 10).expect("int") * mult;

        //will over-count on the 0 boundary; div_euclid follows different edge bounds than the question requires.
        if left && rot == 0 {
            count -= 1;
        }
        let crosses = (rot + rel + if left { -1 } else { 0 }).div_euclid(100);
        let new_rot = (rot + rel).rem_euclid(100);

        count += crosses.abs();

        rot = new_rot;
    }

    return count.to_string();
}
