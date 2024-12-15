#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
            Self::Up => (pos.0, pos.1.saturating_add(1)),
            Self::Right => (pos.0.saturating_add(1), pos.1),
            Self::Down => (pos.0, pos.1.saturating_sub(1)),
            Self::Left => (pos.0.saturating_sub(1), pos.1),
        }
    }

    fn from_char(ch: char) -> Self {
        match ch {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!("Invalid direction character passed."),
        }
    }
}

fn parse_input(raw_input: &str) -> (Vec<Vec<BlockState>>, Vec<Direction>) {
    let (raw_map, raw_instr) = raw_input.split_once("\n\n").expect("Map and Instructions");

    return (vec![], vec![]);
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    return 0.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    return 0.to_string();
}
