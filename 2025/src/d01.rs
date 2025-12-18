use std::{os::unix::raw, panic};

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

        //Backcount as we will overcount.
        if left && rot == 0 {
            count -= 1;
        }
        let crosses = (rot + rel + if left { -1 } else { 0 }).div_euclid(100);
        let new_rot = (rot + rel).rem_euclid(100);

        count += crosses.abs();

        rot = new_rot;
        // println!("{} | {}", rot, count);
    }

    // println!("END OF P1");
    println!("REAL: {}", p2_(&input));
    return count.to_string();
}

pub fn p2_(body: &str) -> String {
    let start = 50;
    let mut pos = start;
    let mut count: i32 = 0;
    let mut i = 0;

    for movement in body.lines() {
        i = i + 1;
        let direction = movement.starts_with("L");
        let mut movement_count: i32 = i32::from_str_radix(&movement[1..], 10).expect("int");
        count += movement_count / 100;
        movement_count = movement_count % 100;
        match direction {
            true => {
                let new_pos = pos - movement_count;
                let corrected_pos = match new_pos {
                    x if x < 0 => {
                        if pos > 0 {
                            // println!("Went through zero, added 1 to the password");
                            count = count + 1;
                        };
                        100 + new_pos
                    }
                    x if x >= 0 => new_pos,
                    _ => panic!(),
                };
                pos = corrected_pos;
            }
            false => {
                let new_pos = pos + movement_count;
                let corrected_pos = match new_pos {
                    x if x <= 99 => new_pos,
                    x if x > 100 => {
                        // println!("Went through zero, added 1 to the password");
                        count = count + 1;
                        new_pos - 100
                    }
                    100 => 0,
                    _ => panic!(),
                };
                pos = corrected_pos;
            }
            _ => panic!(),
        };
        match pos {
            0 => count = count + 1,
            _ => (),
        };
        // println!(
        //     "{}. Moved {} {}, ended up at {}",
        //     i, direction, movement_count, pos
        // );
        // println!("{} | {}", pos, count)
    }
    return count.to_string();
}
