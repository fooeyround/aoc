use pathfinding::prelude::{bfs, count_paths};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &Vec<Vec<u8>>) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
            .iter()
            .filter(|pos| {
                return pos.1 < map.len()
                    && pos.0 < map[0].len()
                    && map[pos.1][pos.0] == 1 + map[y][x];
            })
            .map(|p| *p)
            .collect()
    }
}

fn parse_input(raw_input: &str) -> Vec<Vec<u8>> {
    raw_input
        .split("\n")
        .filter(|f| !f.is_empty())
        .map(|f| {
            f.chars()
                .map(|c| c.to_digit(10).expect("Number") as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let mut sum = 0;

    for (y, itr) in input.iter().enumerate() {
        for (x, &height) in itr.iter().enumerate() {
            if height != 0 {
                continue;
            }

            let mut visited = vec![];

            while let Some(final_pos) = bfs(
                &Pos(x, y),
                |pos| pos.successors(&input),
                |pos| !visited.contains(pos) && input[pos.1][pos.0] == 9,
            ) {
                sum += 1;
                visited.push(*final_pos.last().expect("Value in path"));
            }
        }
    }

    return sum.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let mut sum = 0;

    for (y, itr) in input.iter().enumerate() {
        for (x, &height) in itr.iter().enumerate() {
            if height != 0 {
                continue;
            }

            let count = count_paths(
                Pos(x, y),
                |pos| pos.successors(&input),
                |pos| input[pos.1][pos.0] == 9,
            );
            sum += count;
            //Pathfinding tree to every 9, only going to points 1 up.
        }
    }

    return sum.to_string();
}
