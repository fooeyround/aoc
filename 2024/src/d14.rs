use itertools::Itertools;
use regex::Regex;

const SIZE_X: i64 = 101;
const SIZE_Y: i64 = 103;
const TIME_STEP: i64 = 100;

struct RobotState {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

fn parse_input(raw_input: &str) -> Vec<RobotState> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    raw_input
        .split("\n")
        .filter(|f| !f.trim().is_empty())
        .map(|f| {
            re.captures(f)
                .expect("Pattern conformant")
                .extract::<4>()
                .1
                .iter()
                .map(|v| v.parse::<i64>().expect("Number"))
                .collect::<Vec<i64>>()
        })
        .map(|f| RobotState {
            px: f[0],
            py: f[1],
            vx: f[2],
            vy: f[3],
        })
        .collect()
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let mut quads = [0; 4];

    input
        .iter()
        .map(|robot| {
            (
                (robot.px + robot.vx * TIME_STEP).rem_euclid(SIZE_X),
                (robot.py + robot.vy * TIME_STEP).rem_euclid(SIZE_Y),
            )
        })
        .for_each(|(x, y)| {
            if x == SIZE_X / 2 || y == SIZE_Y / 2 {
                return;
            }
            if x > SIZE_X / 2 {
                if y > SIZE_Y / 2 {
                    quads[0] += 1;
                } else {
                    quads[1] += 1;
                }
            } else {
                if y > SIZE_Y / 2 {
                    quads[2] += 1;
                } else {
                    quads[3] += 1;
                }
            }
        });

    return (quads[0] * quads[1] * quads[2] * quads[3]).to_string();
}

///Explanation: I think it means in a tower? and the only way that is possible is for them to not overlap,
///yes they need to form the shape, but if they do, they will be non overlapping. This could easily fail on an edge case.
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let mut t = 0;

    while input
        .iter()
        .map(|robot| {
            (
                (robot.px + robot.vx * t).rem_euclid(SIZE_X),
                (robot.py + robot.vy * t).rem_euclid(SIZE_Y),
            )
        })
        .duplicates()
        .count()
        != 0
    {
        t += 1;
    }

    return t.to_string();
}
