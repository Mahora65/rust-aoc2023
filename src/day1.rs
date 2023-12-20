use std::{collections::HashMap, fs};

pub fn aoc2023day1() {
    let input = fs::read_to_string("../inputs/day1.txt").unwrap();

    println!("The results of Advent of Code 2023 Day1:");
    println!("    Part I:  {}", part1(&input));
    println!("    Part II: {}", part2(&input));
    println!("-------------------------------------------");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| vec.first().unwrap() * 10 + vec.last().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let paris = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);

    let mut prep_input = input.to_string();
    for (key, val) in paris {
        prep_input = prep_input.replace(key, val);
    }

    part1(&prep_input)
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn test_aoc2023day1part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(input), 142)
    }

    #[test]
    fn test_aoc2023day1part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), 281)
    }
}
