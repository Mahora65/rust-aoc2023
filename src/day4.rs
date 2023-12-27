use std::{collections::HashSet, fs, usize};

pub fn aoc2023day4() {
    let input = fs::read_to_string("inputs/day4.txt").unwrap();

    println!("The results of Advent of Code 2023 Day4:");
    println!("    Part I:  {}", part1(&input));
    println!("    Part II: {}", part2(&input));
    println!("-------------------------------------------");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| line.split(':'))
        .filter(|ele| ele.contains('|'))
        .map(|card| {
            card.split('|')
                .map(|scard| scard.split_whitespace().collect())
                .collect()
        })
        .map(|sets: Vec<HashSet<_>>| sets[0].intersection(&sets[1]).count())
        .filter(|p| p > &0)
        .map(|p| 2_u32.pow(p as u32 - 1))
        .sum()
}
fn part2(input: &str) -> u32 {
    let deck: Vec<(usize, usize)> = input
        .lines()
        .flat_map(|line| line.split(':'))
        .filter(|ele| ele.contains('|'))
        .map(|card| {
            card.split('|')
                .map(|scard| scard.split_whitespace().collect())
                .collect::<Vec<HashSet<&str>>>()
        })
        .map(|sets| sets[0].intersection(&sets[1]).count())
        .enumerate()
        .collect();

    let mut num_cards = vec![1; deck.len()];
    for (i, num_matched) in deck {
        let start = i + 1;
        let end = i + num_matched;
        for j in start..=end {
            if j < num_cards.len() {
                num_cards[j] += num_cards[i];
            }
        }
    }

    num_cards.iter().sum()
}

#[cfg(test)]
mod test {

    use crate::day4::{part1, part2};

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_aoc2023day4_part1() {
        assert_eq!(part1(INPUT), 13);
    }
    #[test]
    fn test_aoc2023day4_par2() {
        assert_eq!(part2(INPUT), 30);
    }
}
