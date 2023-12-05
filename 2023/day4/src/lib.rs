use std::collections::HashMap;

use common::get_input;

pub fn run() {
    println!("Day 4");

    let input = get_input();
    part_one(&input);
    part_two(&input);
}

#[derive(Debug)]
struct Card {
    id: u32,
    wining_numbers: Vec<u32>,
    given_numbers: Vec<u32>,
}

#[derive(Clone)]
struct ProcessedCard {
    id: u32,
    winning_count: u32,
}

struct CardCounter {
    /// <id, count>
    map: HashMap<u32, u32>,
}

impl CardCounter {
    fn new() -> CardCounter {
        CardCounter {
            map: HashMap::new(),
        }
    }
}

impl ProcessedCard {
    fn from_card(card: &Card) -> ProcessedCard {
        let winning_count = card
            .given_numbers
            .iter()
            .filter(|num| card.wining_numbers.contains(num))
            .count() as u32;
        ProcessedCard {
            id: card.id,
            winning_count: winning_count,
        }
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Card> {
    let parsed = input
        .iter()
        .map(|line| {
            let game_split = line.split(":").collect::<Vec<&str>>();

            let game_id = game_split[0].split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            let both_numbers_vec = game_split[1].split("|").collect::<Vec<&str>>();

            let wining_numbers = both_numbers_vec[0]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let given_numbers_raw = both_numbers_vec[1]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Card {
                id: game_id,
                wining_numbers: wining_numbers,
                given_numbers: given_numbers_raw,
            }
        })
        .collect::<Vec<Card>>();

    parsed
}

fn part_one(input: &Vec<String>) {
    //not yet!

    let cards = parse_input(input);

    let result: u32 = cards
        .iter()
        .map(|card| {
            card.given_numbers
                .iter()
                .filter(|num| card.wining_numbers.contains(num))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part_two(input: &Vec<String>) {
    let proccessed_cards = parse_input(input)
        .iter()
        .map(|card| ProcessedCard::from_card(card))
        .collect::<Vec<ProcessedCard>>();

    let mut winning_map: HashMap<u32, u32> = HashMap::new();
    proccessed_cards.iter().for_each(|card| {
        winning_map.insert(card.id, card.winning_count);
    });

    //id to count
    let mut card_counter: HashMap<u32, u32> = HashMap::new();

    //init the card counter with starting cards. You only start with one of each, even if you win multiple times. there would be multiple card instances.
    proccessed_cards.iter().for_each(|card| {
        //deref here does not take form the map right? it passes by value.
        card_counter.insert(card.id, 1);
    });

    let mut current_card_id = 0;
    while card_counter.len() > current_card_id {
        current_card_id += 1;

        let card_win_count = winning_map.get(&(current_card_id as u32)).unwrap_or(&0);

        let card_count = card_counter
            .get(&(current_card_id as u32))
            .unwrap_or(&0)
            .clone();

        for i in (current_card_id + 1)..(card_win_count.clone() as usize + current_card_id + 1) {
            let before_value = card_counter.get(&(i as u32)).unwrap_or(&0);
            card_counter.insert(i as u32, before_value + card_count);
        }
    }

    let sum = card_counter.iter().fold(0, |acc, num| acc + num.1);

    println!("Part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test_input.txt")
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        part_one(&input);

        //Not yet!
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test_input.txt")
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        part_two(&input);

        //Not yet!
    }
}
