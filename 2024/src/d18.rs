use pathfinding::prelude::{bfs, dfs};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &Vec<Pos>, iteraton: usize) -> Vec<(Self, usize)> {
        vec![
            Pos(self.0.saturating_add(1), self.1),
            Pos(self.0.saturating_sub(1), self.1),
            Pos(self.0, self.1.saturating_add(1)),
            Pos(self.0, self.1.saturating_sub(1)),
        ]
        .iter()
        .filter(|pos| pos.0 <= 70 && pos.1 <= 70)
        .filter(|d| {
            map.iter()
                .position(|pos| *pos == **d)
                .map(|dis| dis >= iteraton)
                .unwrap_or(true)
        })
        .map(|f| (*f, iteraton))
        .collect()
    }
}

fn parse_input(raw_input: &str) -> Vec<Pos> {
    raw_input
        .split("\n")
        .filter(|f| !f.is_empty())
        .map(|line| {
            let bb = line.split_once(",").unwrap();
            Pos(
                bb.0.parse::<usize>().unwrap(),
                bb.1.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let path = bfs(
        &(Pos(0, 0), 1024),
        |(pos, iteration)| pos.successors(&input, *iteration),
        |(pos, _it)| *pos == Pos(70, 70),
    )
    .expect("Path");

    return 0.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    return 0.to_string();
}
