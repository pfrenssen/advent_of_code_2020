use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(yolt_adapter_ratings: &Vec<usize>) -> usize {
    let mut ratings = yolt_adapter_ratings.clone();
    let mut current = 0;
    let mut found: (usize, usize, usize) = (0, 0, 1);
    ratings.sort();
    for r in ratings {
        match r - current {
            1 => found.0 += 1,
            2 => found.1 += 1,
            _ => found.2 += 1,
        }
        current = r
    }
    found.0 * found.2
}

#[aoc(day10, part2)]
fn part2(yolt_adapter_ratings: &Vec<usize>) -> u64 {
    let mut ratings = yolt_adapter_ratings.clone();
    let mut adapters: HashMap<usize, u64> = HashMap::new();
    ratings.push(0);
    ratings.sort();
    for r in &ratings {
        let current = adapters.get(r).unwrap_or(&1).clone();
        for i in 1..4 {
            if ratings.contains(&(r + i)) {
                let rating = adapters.entry(r + i).or_insert(0);
                *rating += current;
            }
        }
    }

    *adapters.iter().max().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let test_input = get_test_input();
        let expected = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(expected, parse_input(get_test_input()[0]));
        let expected = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(expected, parse_input(get_test_input()[1]));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input()[0]);
        assert_eq!(35, part1(&input));
        let input = parse_input(get_test_input()[1]);
        assert_eq!(220, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input()[0]);
        assert_eq!(8, part2(&input));
        let input = parse_input(get_test_input()[1]);
        assert_eq!(19208, part2(&input));
    }

    fn get_test_input<'a>() -> Vec<&'a str> {
        vec![
            indoc! {"
                16
                10
                15
                5
                1
                11
                7
                19
                6
                12
                4
            "},
            indoc! {"
                28
                33
                18
                42
                31
                14
                46
                20
                48
                47
                24
                23
                49
                45
                19
                38
                39
                11
                1
                32
                25
                35
                8
                17
                7
                9
                4
                2
                34
                10
                3
            "},
        ]
    }
}
