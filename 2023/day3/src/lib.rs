use common::get_input;

pub fn run() {
    
    
    let input = get_input();


    part_one(&input);
}



struct Position {
    x: usize,
    y: usize,
}

enum EntryType {
    PART(Part),
    SYMBOL(Symbol),
}


struct Part {
    positions: Vec<Position>,
    value: u32,
}

struct Symbol {
    position: Position,
    symbol: char,
}

struct Schematic {
    map: Vec<EntryType>,
}

fn part_one(input: &Vec<String>) {
    let mut sum = 0;




    println!("Part one: {}", sum);
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

            if let Some(number) = c.to_digit(10) {

                let iter: Vec<char> = line.chars().skip(x)
                .take_while(|c| c.is_digit(10))
                .collect();


                let positions: Vec<Position> = iter.iter().enumerate().map(|(i, _)| Position { x: x + i, y }).collect();



                skip = positions.len().clone();
                

                map.push(EntryType::PART(Part { positions, value: number }));

                //skip by positions.len()
                

            } else if c != '.' {
                map.push(EntryType::SYMBOL(Symbol { position: Position { x, y }, symbol: c }));
            } else {
                //Don't worry about . they are suppose to be empty space.
            }

        }
    }

    Schematic { map }
}