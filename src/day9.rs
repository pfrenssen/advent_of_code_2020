use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashSet;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1_puzzle(data: &Vec<usize>) -> usize {
    part1(data, 25)
}

fn part1(code: &Vec<usize>, preamble_length: usize) -> usize {
    for i in preamble_length..code.len() {
        if !sum_exists(&code[i - preamble_length..i], code[i]) {
            return code[i];
        }
    }
    unreachable!()
}

fn sum_exists(data: &[usize], sum: usize) -> bool {
    for i in 0..data.len() - 1 {
        for j in i + 1..data.len() {
            if data[i] + data[j] == sum {
                return true;
            }
        }
    }
    false
}

#[aoc(day9, part2)]
fn part2_puzzle(data: &Vec<usize>) -> usize {
    part2(data, 25)
}

fn part2(data: &Vec<usize>, preamble_length: usize) -> usize {
    let sum = part1(data, preamble_length);
    for i in 0..data.len() - 1 {
        if let Ok((min, max)) = get_range_bounds(&data[i..data.len()], sum) {
            return min + max;
        }
    }
    unreachable!()
}

fn get_range_bounds(data: &[usize], sum: usize) -> Result<(usize, usize), ()> {
    let mut result = 0;
    for i in 0..data.len() {
        result = result + data[i];
        if result == sum {
            let range = &data[0..i + 1];
            return Ok((*range.iter().min().unwrap(), *range.iter().max().unwrap()));
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input());
        assert_eq!(127, part1(&input, 5));

        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 277, 127, 219, 299, 277,
            309, 576,
        ];
        assert_eq!(127, part1(&input, 5));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input());
        assert_eq!(62, part2(&input, 5));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        "}
    }
}
