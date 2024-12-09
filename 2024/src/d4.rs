const DIRECTIONS: [(i64, i64); 8] = [
    (-1, 1),
    (1, 1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, -1),
    (-1, -1),
];

pub fn solve1(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    const WORD: &str = "XMAS";
    let arr: Vec<Vec<char>> = input.iter().map(|f| f.chars().collect()).collect();

    let mut sum = 0;

    for idxy in 0..arr.len() {
        for idxx in 0..arr[idxy].len() {
            for (dx, dy) in DIRECTIONS {
                let mut fail = false;
                for cr in 0..WORD.len() {
                    let ny = cr as i64 * dy + idxy as i64;
                    let nx = cr as i64 * dx + idxx as i64;
                    if !(ny >= 0
                        && ny < arr.len() as i64
                        && nx >= 0
                        && nx < arr[idxy].len() as i64
                        && arr[ny as usize][nx as usize] == WORD.chars().nth(cr).unwrap())
                    {
                        fail = true;
                        break;
                    }
                }
                if !fail {
                    sum += 1;
                }
            }
        }
    }

    return sum.to_string();
}

pub fn solve2(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    const WORD: &str = "MAS";
    const WORD_REV: &str = "SAM";
    let arr: Vec<Vec<char>> = input.iter().map(|f| f.chars().collect()).collect();

    let mut sum = 0;

    for idxy in 0..arr.len() {
        for idxx in 0..arr[idxy].len() {
            if !(arr[idxy][idxx] == 'A'
                && idxy + 1 < arr.len()
                && idxy > 0
                && idxx + 1 < arr[idxy].len()
                && idxx > 0)
            {
                continue;
            }

            let a_diag = [
                arr[idxy - 1][idxx - 1],
                arr[idxy][idxx],
                arr[idxy + 1][idxx + 1],
            ]
            .iter()
            .collect::<String>();
            let b_diag = [
                arr[idxy + 1][idxx - 1],
                arr[idxy][idxx],
                arr[idxy - 1][idxx + 1],
            ]
            .iter()
            .collect::<String>();

            if (a_diag == WORD || a_diag == WORD_REV) && (b_diag == WORD || b_diag == WORD_REV) {
                sum += 1;
            }
        }
    }

    return sum.to_string();
}
