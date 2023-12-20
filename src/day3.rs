use std::{char, collections::HashSet, fs, usize};

pub fn aoc2023day3() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();

    println!("The results of Advent of Code 2023 Day3:");
    println!("    Part I:  {}", part1(&input));
    // println!("    Part II: {}", part2(&input));
    println!("-------------------------------------------");
}

fn part1(input: &str) -> u32 {
    let rs: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sym_pos = HashSet::new();
    let mut num_pos = HashSet::new();
    let mut nums = Vec::new();

    // looking for a position of the symbols
    for (i, line) in rs.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_ascii_digit() && ch != &'.' {
                sym_pos.insert((i as i32, j as i32));
            }
        }
    }

    // looking around the symbols
    for (i, j) in &sym_pos {
        for row in i - 1..=i + 1 {
            for col in j - 1..=j + 1 {
                if row >= 0
                    && row <= rs[0].len() as i32
                    && col >= 0
                    && col <= rs[0].len() as i32
                    && rs[row as usize][col as usize].is_ascii_digit()
                {
                    let mut ncol = col;
                    let mut cur = (row, col);
                    while ncol >= 0 && rs[row as usize][ncol as usize].is_ascii_digit() {
                        cur = (row, ncol);
                        ncol -= 1;
                    }
                    num_pos.insert(cur);
                }
            }
        }
    }

    for (i, j) in num_pos {
        let mut cur = String::from("");
        let mut ncol = j;
        while ncol < rs[0].len() as i32 && rs[i as usize][ncol as usize].is_ascii_digit() {
            cur.push(rs[i as usize][ncol as usize]);
            ncol += 1;
        }
        nums.push(cur);
    }

    nums.iter().map(|ele| ele.parse::<u32>().unwrap()).sum()
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let rs: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sym_pos = HashSet::new();
    // looking for a position of the symbols
    for (i, line) in rs.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_ascii_digit() && ch == &'*' {
                sym_pos.insert((i as i32, j as i32));
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use crate::day3::{part1, part2};

    #[test]
    fn test_aoc2023day3part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(&input), 4361)
    }

    #[test]
    fn test_aoc2023day3part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(&input), 467835)
    }
}
