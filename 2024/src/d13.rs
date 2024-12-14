fn parse_input(raw_input: &str) -> Vec<Vec<u64>> {
    raw_input
        .split("\n\n")
        .filter(|f| !f.trim().is_empty())
        .map(|f| {
            f.chars()
                .map(|f| if f.is_digit(10) { f } else { ' ' })
                .collect::<String>()
                .split_whitespace()
                .map(|f| f.parse::<u64>().expect("Number"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);
    let sum: i64 = input
        .iter()
        .filter_map(|f| {
            min_cost_by_cramer_rule(
                f[0] as i64,
                f[2] as i64,
                f[1] as i64,
                f[3] as i64,
                f[4] as i64,
                f[5] as i64,
            )
        })
        .sum();

    return sum.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);
    let sum: i64 = input
        .iter()
        .filter_map(|f| {
            min_cost_by_cramer_rule(
                f[0] as i64,
                f[2] as i64,
                f[1] as i64,
                f[3] as i64,
                f[4] as i64 + 10000000000000,
                f[5] as i64 + 10000000000000,
            )
        })
        .sum();

    return sum.to_string();
}

//https://en.wikipedia.org/wiki/Cramer%27s_rule
fn min_cost_by_cramer_rule(
    a_x: i64,
    b_x: i64,
    a_y: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
) -> Option<i64> {
    // Calculate determinant
    let det = a_x * b_y - b_x * a_y;
    // Check if the determinant is zero
    if det == 0 {
        panic!("Found non-invertible matrix in input");
    }
    // Calculate adjugate matrix elements
    let adj_a = b_y;
    let adj_b = -b_x;
    let adj_c = -a_y;
    let adj_d = a_x;
    // Calculate adjugate prizes
    let adj_prize_x = adj_a * prize_x + adj_b * prize_y;
    let adj_prize_y = adj_c * prize_x + adj_d * prize_y;
    // Check if both adjugate prizes are divisible by determinant
    if adj_prize_x % det == 0 && adj_prize_y % det == 0 {
        let adj_prize_x_quot = adj_prize_x / det;
        let adj_prize_y_quot = adj_prize_y / det;
        Some(3 * adj_prize_x_quot + adj_prize_y_quot)
    } else {
        None
    }
}
