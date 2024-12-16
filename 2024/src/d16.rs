#[derive(Clone, Copy, Hash, Debug, PartialEq, PartialOrd)]
struct DirectedPosition((usize, usize), Direction);

#[derive(Clone, Copy, Hash, Debug, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn rotate_left(&self) -> Self {
        return self.opposite().rotate_right();
    }
    fn opposite(&self) -> Self {
        return self.rotate_right().rotate_right();
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

impl DirectedPosition {
    fn successors(&self, map: &Vec<Vec<bool>>) -> Vec<(DirectedPosition, usize)> {
        vec![
            (DirectedPosition(self.1.offset_of(self.0), self.1), 1),
            (
                DirectedPosition(self.1.rotate_left().offset_of(self.0), self.1.rotate_left()),
                1001,
            ),
            (
                DirectedPosition(
                    self.1.rotate_right().offset_of(self.0),
                    self.1.rotate_right(),
                ),
                1001,
            ),
        ]
        .iter()
        .filter(|d| !map[d.0 .0 .1][d.0 .0 .0])
        .map(|f| *f)
        .collect()
    }
}

fn parse_input(raw_input: &str) -> (Vec<Vec<bool>>, DirectedPosition, DirectedPosition) {
    let mut start_pos = DirectedPosition((0, 0), Direction::Right);
    let mut end_pos = DirectedPosition((0, 0), Direction::Right);

    (
        raw_input
            .split("\n")
            .enumerate()
            .filter(|(y, f)| !f.is_empty())
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start_pos = DirectedPosition((x, y), Direction::Right);
                            false
                        }
                        'E' => {
                            end_pos = DirectedPosition((x, y), Direction::Right);
                            false
                        }
                        '#' => true,
                        _ => false,
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>(),
        start_pos,
        end_pos,
    )
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    return 0.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    return 0.to_string();
}
