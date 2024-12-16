#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlockState {
    Box,
    Wall,
    Air,
}

impl BlockState {
    fn from_char(ch: char) -> Self {
        match ch {
            'O' => BlockState::Box,
            '#' => BlockState::Wall,
            '.' => BlockState::Air,
            _ => unreachable!("Invalid map character passed."),
        }
    }
}

impl Direction {
    fn offset_of(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (pos.0, pos.1.saturating_sub(1)),
            Self::Right => (pos.0.saturating_add(1), pos.1),
            Self::Down => (pos.0, pos.1.saturating_add(1)),
            Self::Left => (pos.0.saturating_sub(1), pos.1),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
}

fn sim_map_p1(
    mut map: Vec<Vec<BlockState>>,
    instructions: Vec<Direction>,
    mut player_pos: (usize, usize),
) -> Vec<Vec<BlockState>> {
    'outer: for instruction_direction in instructions {
        let mut search_pos = instruction_direction.offset_of(player_pos);

        if map[search_pos.1][search_pos.0] == BlockState::Air {
            player_pos = search_pos;
            continue;
        }

        while map[search_pos.1][search_pos.0] != BlockState::Air {
            if map[search_pos.1][search_pos.0] == BlockState::Wall {
                continue 'outer;
            }
            search_pos = instruction_direction.offset_of(search_pos);
        }

        let mut n_search = search_pos;

        while n_search != instruction_direction.offset_of(player_pos) {
            let tmp = map[n_search.1][n_search.0];
            let new_pos = instruction_direction.opposite().offset_of(n_search);
            map[n_search.1][n_search.0] = map[new_pos.1][new_pos.0];
            map[new_pos.1][new_pos.0] = tmp;

            n_search = instruction_direction.opposite().offset_of(n_search);
        }
        let player_direct = instruction_direction.offset_of(player_pos);
        player_pos = player_direct;
    }
    return map;
}

fn parse_input(raw_input: &str) -> (Vec<Vec<BlockState>>, Vec<Direction>, (usize, usize)) {
    let (raw_map, raw_instr) = raw_input.split_once("\n\n").expect("Map and Instructions");

    let mut player_pos = (0, 0);
    (
        raw_map
            .split("\n")
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '@' => {
                            player_pos = (x, y);
                            BlockState::Air
                        }
                        chr => BlockState::from_char(chr),
                    })
                    .collect::<Vec<BlockState>>()
            })
            .collect(),
        raw_instr
            .chars()
            .filter_map(|f| Direction::from_char(f))
            .collect(),
        player_pos,
    )
}

pub fn solve1(raw_input: &str) -> String {
    let (map, inst, player_pos) = parse_input(raw_input);
    let map = sim_map_p1(map, inst, player_pos);

    let sum: usize = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, block)| match block {
                    BlockState::Box => y * 100 + x,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum();

    sum.to_string()
}

pub fn solve2(raw_input: &str) -> String {
    0.to_string()
}
