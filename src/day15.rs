use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day15, part1)]
fn part1(starting_numbers: &Vec<usize>) -> usize {
    get_number(starting_numbers, 2020)
}

#[aoc(day15, part2)]
fn part2(starting_numbers: &Vec<usize>) -> usize {
    get_number(starting_numbers, 30000000)
}

fn get_number(starting_numbers: &Vec<usize>, number: usize) -> usize {
    let mut spoken_numbers: HashMap<usize, (usize, Option<usize>)> = HashMap::new();
    let mut to_speak: usize = 0;
    let mut i = 1;
    for number in starting_numbers {
        spoken_numbers.insert(*number, (i, None));
        to_speak = *number;
        i += 1;
    }

    for i in i..=number {
        to_speak = match spoken_numbers.get(&to_speak) {
            Some((n, Some(pn))) => n - pn,
            _ => 0,
        };

        let to_insert = match spoken_numbers.get(&to_speak) {
            Some((n, _)) => (i, Some(*n)),
            None => (i, None),
        };

        spoken_numbers.insert(to_speak, to_insert);
    }
    to_speak
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![0, 3, 6];
        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn test_part1() {
        let test_cases = vec![
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];
        for test_case in test_cases {
            assert_eq!(test_case.1, part1(&parse_input(test_case.0)));
        }
    }

    #[test]
    #[ignore] // This test is quite slow.
    fn test_part2() {
        let test_cases = vec![
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];
        for test_case in test_cases {
            assert_eq!(test_case.1, part2(&parse_input(test_case.0)));
        }
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            0,3,6
        "}
    }
}
