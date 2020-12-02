use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
            let caps = re.captures(l).unwrap();
            Line {
                min: caps[1].parse().unwrap(),
                max: caps[2].parse().unwrap(),
                letter: caps[3].chars().collect::<Vec<char>>()[0],
                password: caps[4].to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(lines: &[Line]) -> i32 {
    lines.iter().fold(0, |acc, l: &Line| {
        let count = l.password.matches(l.letter).count();
        let range = l.min..=l.max;
        match range.contains(&count) {
            true => acc + 1,
            false => acc,
        }
    })
}

#[aoc(day2, part2)]
fn part2(lines: &[Line]) -> i32 {
    lines.iter().fold(0, |acc, l: &Line| {
        let mut matches = 0;
        for pos in &[l.min, l.max] {
            if l.password.len() >= *pos {
                if l.password.chars().collect::<Vec<char>>()[*pos - 1] == l.letter {
                    matches = matches + 1;
                }
            }
        }
        match matches {
            1 => acc + 1,
            _ => acc,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[
                Line {
                    min: 1,
                    max: 3,
                    letter: 'a',
                    password: "abcde".to_string()
                },
                Line {
                    min: 1,
                    max: 3,
                    letter: 'b',
                    password: "cdefg".to_string()
                },
                Line {
                    min: 2,
                    max: 9,
                    letter: 'c',
                    password: "ccccccccc".to_string()
                },
            ]),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[
                Line {
                    min: 1,
                    max: 3,
                    letter: 'a',
                    password: "abcde".to_string()
                },
                Line {
                    min: 1,
                    max: 3,
                    letter: 'b',
                    password: "cdefg".to_string()
                },
                Line {
                    min: 2,
                    max: 9,
                    letter: 'c',
                    password: "ccccccccc".to_string()
                },
            ]),
            1
        );
    }
}
