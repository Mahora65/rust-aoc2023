use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Bag {
    red: u32,
    blue: u32,
    green: u32,
}
#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn parse(input: &str) -> Self {
        let t: Vec<&str> = input.split(':').collect();
        let id = t[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let rounds: Vec<Round> = t[1].split(';').map(Round::parse).collect();

        Self { id, rounds }
    }

    pub fn check_possible(&self, bag: &Bag) -> bool {
        let vec = &self.rounds;
        vec.iter()
            .map(|draw| draw.check_possible(bag))
            .all(|draw| draw)
    }

    pub fn cal_power(&self) -> u32 {
        let vec = &self.rounds;
        let red_cube = vec.iter().map(|round| round.red).max().unwrap_or(1);
        let blue_cube = vec.iter().map(|round| round.blue).max().unwrap_or(1);
        let green_cube = vec.iter().map(|round| round.green).max().unwrap_or(1);

        red_cube * blue_cube * green_cube
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl Round {
    pub fn new(input: &HashMap<&str, u32>) -> Self {
        Self {
            red: *input.get(&"red").unwrap_or(&0),
            blue: *input.get("blue").unwrap_or(&0),
            green: *input.get("green").unwrap_or(&0),
        }
    }

    pub fn check_possible(&self, bag: &Bag) -> bool {
        bag.red >= self.red && bag.blue >= self.blue && bag.green >= self.green
    }

    // pub fn parse(input: &str) -> Self {
    pub fn parse(input: &str) -> Self {
        let t: Vec<Vec<&str>> = input
            .split(',')
            .map(|draw| draw.rsplit(' ').collect())
            .collect();
        let mut cotr_hash: HashMap<&str, u32> = HashMap::new();

        for ele in t {
            cotr_hash.insert(ele[0], ele[1].parse().unwrap());
        }

        Self::new(&cotr_hash)
    }
}

pub fn aoc2023day2() {
    let input = fs::read_to_string("../inputs/day2.txt").unwrap();

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    println!("The results of Advent of Code 2023 Day2:");
    println!("    Part I:  {}", part1(&bag, &input));
    println!("    Part II: {}", part2(&input));
    println!("-------------------------------------------");
}

fn part1(bag: &Bag, input: &str) -> u32 {
    let mut score: u32 = 0;
    let games = input.lines().map(Game::parse).collect::<Vec<Game>>();
    for game in games {
        if game.check_possible(bag) {
            score += game.id;
        }
    }
    score
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Game::parse)
        .map(|game| game.cal_power())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day2::{part1, part2, Bag};

    use super::Round;

    #[test]
    fn test_aoc2023day2part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(part1(&bag, input), 8)
    }

    #[test]
    fn test_aoc2023day2part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part2(input), 2286)
    }

    #[test]
    fn test_round_cotr() {
        let input: HashMap<&str, u32> = HashMap::from([("red", 1), ("blue", 2), ("green", 3)]);
        let input2: HashMap<&str, u32> = HashMap::from([("blue", 2), ("green", 3)]);

        let round = Round::new(&input);
        let round2 = Round::new(&input2);

        assert_eq!(round.red, *input.get("red").unwrap());
        assert_eq!(round.blue, *input.get("blue").unwrap());
        assert_eq!(round.green, *input.get("green").unwrap());
        assert_eq!(round2.red, 0);
        assert_eq!(round2.blue, *input.get("blue").unwrap());
        assert_eq!(round2.green, *input.get("green").unwrap());
    }

    #[test]
    fn test_round_parse() {
        // let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let input = "3 blue, 4 red";

        let round = Round::parse(input);

        assert_eq!(round.blue, 3);
        assert_eq!(round.red, 4);
        assert_eq!(round.green, 0);
    }

    #[test]
    fn test_round_check_possible() {
        let bag_possible = Bag {
            red: 4,
            blue: 4,
            green: 4,
        };
        let bag_possible2 = Bag {
            red: 3,
            blue: 3,
            green: 3,
        };
        let bag_impossible = Bag {
            red: 2,
            blue: 3,
            green: 2,
        };

        let input: HashMap<&str, u32> = HashMap::from([("red", 3), ("blue", 3), ("green", 3)]);
        let round1 = Round::new(&input);

        assert_eq!(round1.check_possible(&bag_impossible), false);
        assert_eq!(round1.check_possible(&bag_possible), true);
        assert_eq!(round1.check_possible(&bag_possible2), true);
    }
}
