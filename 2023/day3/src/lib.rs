use common::get_input;

pub fn run() {
    let input = get_input();

    part_one(&input);
    part_two(&input);
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum EntryType {
    PART(Part),
    SYMBOL(Symbol),
}

#[derive(Debug)]
struct Part {
    positions: Vec<Position>,
    value: u32,
}

#[derive(Debug)]
struct Symbol {
    position: Position,
    symbol: char,
}

#[derive(Debug)]
struct Schematic {
    map: Vec<EntryType>,
}

fn part_one(input: &Vec<String>) {
    let schematic = parse_input(input);

    let symbols = schematic
        .map
        .iter()
        .filter_map(|entry| match entry {
            EntryType::SYMBOL(symbol) => Some(symbol),
            _ => None,
        })
        .collect::<Vec<&Symbol>>();

    let parts = schematic
        .map
        .iter()
        .filter_map(|entry| match entry {
            EntryType::PART(part) => Some(part),
            _ => None,
        })
        .collect::<Vec<&Part>>();

    let touching_parts = parts
        .iter()
        .filter(|part| {
            symbols
                .iter()
                .any(|symbol| is_part_touching_pos(part, &symbol.position))
        })
        .collect::<Vec<&&Part>>();

    let sum: u64 = touching_parts
        .iter()
        .fold(0, |acc, part| acc + part.value as u64);

    println!("Part one: {}", sum);
}

fn part_two(input: &Vec<String>) {
    let schematic = parse_input(input);

    let symbols = schematic
        .map
        .iter()
        .filter_map(|entry| match entry {
            EntryType::SYMBOL(symbol) => Some(symbol),
            _ => None,
        })
        .collect::<Vec<&Symbol>>();

    let parts = schematic
        .map
        .iter()
        .filter_map(|entry| match entry {
            EntryType::PART(part) => Some(part),
            _ => None,
        })
        .collect::<Vec<&Part>>();

    let symbols_touching_exactly_two_parts = symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .filter_map(|symbol| {
            let connected_part_values = parts
                .iter()
                .filter_map(|part| {
                    if is_part_touching_pos(part, &symbol.position) {
                        return Some(part.value);
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<u32>>();
            if connected_part_values.len() == 2 {
                return Some((connected_part_values[0], connected_part_values[1]));
            } else {
                return None;
            }
        })
        .collect::<Vec<(u32, u32)>>();

    let sum: u64 = symbols_touching_exactly_two_parts
        .iter()
        .map(|values| values.0 as u64 * values.1 as u64)
        .sum();

    println!("Part two: {:?}", sum);
}

fn parse_input(input: &Vec<String>) -> Schematic {
    let mut map: Vec<EntryType> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut skip = 0;
        for (x, c) in line.chars().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            if let Some(_) = c.to_digit(10) {
                let iter: Vec<char> = line
                    .chars()
                    .skip(x)
                    .take_while(|c| c.is_digit(10))
                    .collect();

                let positions: Vec<Position> = iter
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Position { x: x + i, y })
                    .collect();

                let folded_value_string = iter.iter().fold(String::new(), |mut acc, c| {
                    acc.push(*c);
                    acc
                });

                let parsed_value = match folded_value_string.parse::<u32>() {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Error: {}", err);
                        0
                    }
                };

                skip = positions.len().clone() - 1;

                map.push(EntryType::PART(Part {
                    positions,
                    value: parsed_value,
                }));

                //skip by positions.len()
            } else if c != '.' {
                map.push(EntryType::SYMBOL(Symbol {
                    position: Position { x, y },
                    symbol: c,
                }));
            } else {
                //Don't worry about . they are suppose to be empty space.
            }
        }
    }

    Schematic { map }
}

fn distance_between(a: usize, b: usize) -> u32 {
    ((a as i64 - b as i64).abs()) as u32
}

fn is_part_touching_pos(part: &Part, position: &Position) -> bool {
    part.positions.iter().any(|part_pos| {
        distance_between(part_pos.x, position.x) <= 1
            && distance_between(position.y, part_pos.y) <= 1
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = include_str!("test_input.txt")
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let schematic = parse_input(&input);

        //rebuilt input
        let mut rebuilt_input = String::new();

        for y in 0..=10 {
            for x in 0..=10 {
                let mut found = false;
                for entry in schematic.map.iter() {
                    match entry {
                        EntryType::SYMBOL(symbol) => {
                            if symbol.position.x == x && symbol.position.y == y {
                                rebuilt_input.push(symbol.symbol);
                                found = true;
                                break;
                            }
                        }
                        EntryType::PART(part) => {
                            if part.positions.iter().any(|pos| pos.x == x && pos.y == y) {
                                rebuilt_input.push_str(&part.value.to_string());
                                found = true;
                                break;
                            }
                        }
                    }
                }

                if !found {
                    rebuilt_input.push('.');
                }
            }
            rebuilt_input.push('\n');
        }

        // println!("{:?}", &schematic)
    }
}
