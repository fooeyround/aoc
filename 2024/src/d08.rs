pub fn solve1(raw_input: &str) -> String {
    let input: Vec<Vec<char>> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.chars().collect())
            .collect()
    };

    let mut antinodes: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];

    for (y1, innr) in input.iter().enumerate() {
        for (x1, val1) in innr.iter().enumerate() {
            if *val1 == '.' {
                continue;
            }
            for (y2, innr) in input.iter().enumerate() {
                for (x2, val2) in innr.iter().enumerate() {
                    if *val2 == '.' || val1 != val2 || (x1 == x2 && y1 == y2) {
                        continue;
                    }

                    if 2 * y1 - y2 < antinodes.len() && 2 * x1 - x2 < antinodes[0].len() {
                        antinodes[2 * y1 - y2][2 * x1 - x2] = true;
                    }
                }
            }
        }
    }

    return antinodes
        .iter()
        .map(|innr| innr.iter().filter(|b| **b).count())
        .sum::<usize>()
        .to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input: Vec<Vec<char>> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.chars().collect())
            .collect()
    };

    let mut antinodes: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];

    for (y1, innr) in input.iter().enumerate() {
        for (x1, val1) in innr.iter().enumerate() {
            if *val1 == '.' {
                continue;
            }
            for (y2, innr) in input.iter().enumerate() {
                for (x2, val2) in innr.iter().enumerate() {
                    if *val2 == '.' || val1 != val2 || (x1 == x2 && y1 == y2) {
                        continue;
                    }

                    for i in 0..=(antinodes.len() * antinodes[0].len() / 2) {
                        if (y1 - i * (y2 - y1)) < antinodes.len()
                            && (x1 - i * (x2 - x1)) < antinodes[0].len()
                        {
                            antinodes[y1 - i * (y2 - y1)][x1 - i * (x2 - x1)] = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    return antinodes
        .iter()
        .map(|innr| innr.iter().filter(|b| **b).count())
        .sum::<usize>()
        .to_string();
}
