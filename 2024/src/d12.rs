pub fn parse_input(raw_input: &str) -> Vec<Vec<char>> {
    raw_input
        .split("\n")
        .filter(|f| !f.is_empty())
        .map(|f| f.chars().collect())
        .collect()
}

fn count_peri_area(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<(usize, usize)>,
    pos: (usize, usize),
    plant: char,
) -> (usize, usize) {
    if pos.1 >= map.len() || pos.0 >= map[0].len() || plant != map[pos.1][pos.0] {
        return (1, 0);
    }
    if visited.contains(&pos) {
        return (0, 0);
    }
    visited.push(pos);

    let mut pa = (0, 0);
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = (pos.0 as isize + x) as usize;
        let new_y = (pos.1 as isize + y) as usize;

        let (peri, area) = count_peri_area(&map, visited, (new_x, new_y), plant);
        pa.0 += peri;
        pa.1 += area;
    }
    //Do self.

    pa.1 += 1;
    return (pa.0, pa.1);
}

pub fn solve1(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let mut visited: Vec<(usize, usize)> = vec![];

    let mut sum = 0;

    for (y, itr) in input.iter().enumerate() {
        for (x, plant) in itr.iter().enumerate() {
            let (peri, area) = count_peri_area(&input, &mut visited, (x, y), *plant);
            sum += peri * area;
        }
    }

    return sum.to_string();
}

fn count_peri_area2(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<(usize, usize)>,
    pos: (usize, usize),
    plant: char,
    last_pos: (usize, usize),
) -> (usize, usize) {
    if pos.1 >= map.len() || pos.0 >= map[0].len() || plant != map[pos.1][pos.0] {
        let (pos_control_side, fallback_ending) = if last_pos.0 != pos.0 {
            ((last_pos.0, last_pos.1 + 1), (pos.0, last_pos.1 + 1))
        } else if last_pos.1 != pos.1 {
            ((last_pos.0 + 1, last_pos.1), (last_pos.0 + 1, pos.1))
        } else {
            return (0, 0);
        };

        if pos_control_side.1 >= map.len()
            || pos_control_side.0 >= map[0].len()
            || plant != map[pos_control_side.1][pos_control_side.0]
            || (fallback_ending.1 < map.len()
                && fallback_ending.0 < map[0].len()
                && plant == map[fallback_ending.1][fallback_ending.0])
        {
            return (1, 0);
        }

        return (0, 0);
    }
    if visited.contains(&pos) {
        return (0, 0);
    }
    visited.push(pos);

    let mut pa = (0, 0);
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = (pos.0 as isize + x) as usize;
        let new_y = (pos.1 as isize + y) as usize;

        let (peri, area) = count_peri_area2(&map, visited, (new_x, new_y), plant, pos);
        pa.0 += peri;
        pa.1 += area;
    }
    //Do self.

    pa.1 += 1;
    return (pa.0, pa.1);
}

pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    let mut visited: Vec<(usize, usize)> = vec![];

    let mut sum = 0;

    for (y, itr) in input.iter().enumerate() {
        for (x, plant) in itr.iter().enumerate() {
            let (peri, area) = count_peri_area2(&input, &mut visited, (x, y), *plant, (0, 0));
            sum += peri * area;
        }
    }

    return sum.to_string();
}
