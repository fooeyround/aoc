use std::primitive;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn opposite(&self) -> Self {
        return self.turn_right().turn_right();
    }
    fn turn_left(&self) -> Direction {
        return self.turn_right().turn_right().turn_right();
    }
    fn from_char(ch: char) -> Self {
        match ch {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!("No other char should be passed!"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos2d(usize, usize);

impl Pos2d {
    //This is a special case where top left is origin.
    fn offset_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self(self.0, self.1 - 1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}
#[derive(Debug)]
struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    fn parse(raw_input: &str) -> (Self, Pos2d, Direction) {
        let mut pos = Pos2d(0, 0);
        let mut dir = Direction::Up;

        let raw_map = raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .enumerate()
            .map(|(y, f)| {
                f.chars()
                    .enumerate()
                    .map(|(x, chr)| match chr {
                        '^' | '>' | 'v' | '<' => {
                            dir = Direction::from_char(chr);
                            pos = Pos2d(x, y);
                            false
                        }
                        '#' => true,
                        _ => false,
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();

        return (Map { map: raw_map }, pos, dir);
    }

    fn is_on_map(&self, pos: &Pos2d) -> bool {
        return pos.1 < self.map.len() && pos.0 < self.map[pos.1].len();
    }
    fn pos_is_blocked(&self, pos: Pos2d) -> bool {
        if self.is_on_map(&pos) {
            return self.map[pos.1][pos.0];
        } else {
            false
        }
    }
}

pub fn solve1(raw_input: &str) -> String {
    let (map, posi, diri) = Map::parse(raw_input);

    let mut block_map = vec![vec![false; map.map[0].len()]; map.map.len()];

    {
        let mut pos = posi.clone();
        let mut dir = diri.clone();

        loop {
            block_map[pos.1][pos.0] = true;
            if map.pos_is_blocked(pos.offset_dir(dir)) {
                dir = dir.turn_right();
            }
            if !map.pos_is_blocked(pos.offset_dir(dir)) {
                pos = pos.offset_dir(dir);
            }
            if !map.is_on_map(&pos) {
                break;
            }
        }
    }

    // {
    //     let mut pos = posi.clone();
    //     let mut dir = diri.clone();

    //     loop {
    //         block_map[pos.1][pos.0] = true;
    //         if map.pos_is_blocked(pos.offset_dir(dir.turn_left())) {
    //             dir = dir.turn_left();
    //         }

    //         pos = pos.offset_dir(dir.opposite());

    //         if !map.is_on_map(&pos) {
    //             break;
    //         }
    //     }
    // }

    return block_map
        .iter()
        .map(|col| col.iter().filter(|visited| **visited).count())
        .sum::<usize>()
        .to_string();
}
pub fn solve2(raw_input: &str) -> String {
    return String::new();
}
