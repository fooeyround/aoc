use common::get_input;

#[derive(Debug)]
struct RedGreenBlue {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<RedGreenBlue>,
}

pub fn run() {
    //Entry point to the day's code

    let input = get_input();

    part_one(&input);

    part_two(&input);
}

fn part_one(input: &Vec<String>) {
    let games: Vec<Game> = parse_games(input);

    let max_colors = RedGreenBlue {
        red: 12,
        green: 13,
        blue: 14,
    };

    //filter games based on rule that you could not be shown more than there should be.

    let valid_games = games
        .iter()
        .filter(|game| {
            game.cubes.iter().all(|cube| {
                cube.red <= max_colors.red
                    && cube.green <= max_colors.green
                    && cube.blue <= max_colors.blue
            })
        })
        .collect::<Vec<&Game>>();

    let valid_ids = valid_games.iter().map(|game| game.id).collect::<Vec<u32>>();

    //sum those ids.

    let sum: u32 = valid_ids.iter().fold(0, |acc, id: &u32| acc + id);

    println!("Sum part1: {}", sum);
}

fn part_two(input: &Vec<String>) {
    let games: Vec<Game> = parse_games(input);

    let pow_sum: u32 = games
        .iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in game.cubes.iter() {
                red = red.max(cube.red);
                green = green.max(cube.green);
                blue = blue.max(cube.blue);
            }

            let power = red * green * blue;

            power
        })
        .sum();

    println!("Sum part2: {}", pow_sum);
}

fn parse_games(input: &Vec<String>) -> Vec<Game> {
    let games: Vec<Game> = input
        .iter()
        .map(|raw_line| {
            let mut game = Game {
                id: 0,
                cubes: Vec::new(),
            };
            let game_id_split: Vec<&str> = raw_line.split(':').collect();
            let game_id = game_id_split[0].split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            game.id = game_id;
            for group in game_id_split[1].split(";") {
                let rgb_split: Vec<&str> = group.split(",").collect();

                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                rgb_split.iter().for_each(|rgb| {
                    let color_split: Vec<&str> = rgb.split_whitespace().collect();

                    let value = color_split[0].parse::<u32>().unwrap();
                    let color = color_split[1];
                    if color == "red" {
                        red += value;
                    } else if color == "green" {
                        green += value;
                    } else if color == "blue" {
                        blue += value;
                    }
                });
                game.cubes.push(RedGreenBlue {
                    red: red,
                    green: green,
                    blue: blue,
                });
            }

            game
        })
        .collect();

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("test_input.txt")
            .lines()
            .map(|f| f.to_owned())
            .collect::<Vec<String>>();

        let games = parse_games(&input);
    }
}
