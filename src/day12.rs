use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(char, isize)> {
    let re = Regex::new(r"^([NSFWLER])(\d+)$").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (caps[1].parse::<char>().unwrap(), caps[2].parse().unwrap())
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(navigation: &Vec<(char, isize)>) -> isize {
    let mut coords = (0isize, 0isize);
    let mut direction = 90;
    for (instruction, value) in navigation {
        match *instruction {
            'N' => coords.1 -= value,
            'E' => coords.0 += value,
            'S' => coords.1 += value,
            'W' => coords.0 -= value,
            'L' => direction = (720 + direction - value) % 360,
            'R' => direction = (direction + value) % 360,
            _ => match direction {
                0 => coords.1 -= value,
                90 => coords.0 += value,
                180 => coords.1 += value,
                270 => coords.0 -= value,
                _ => unreachable!(),
            },
        }
    }
    coords.0.abs() + coords.1.abs()
}

#[aoc(day12, part2)]
fn part2(navigation: &Vec<(char, isize)>) -> isize {
    let mut coords = (0isize, 0isize);
    let mut waypoint = (10isize, -1isize);
    for (instruction, value) in navigation {
        match *instruction {
            'N' => waypoint.1 -= value,
            'E' => waypoint.0 += value,
            'S' => waypoint.1 += value,
            'W' => waypoint.0 -= value,
            'L' => match value {
                90 => waypoint = (waypoint.1, -waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (-waypoint.1, waypoint.0),
                _ => unreachable!(),
            },
            'R' => match value {
                90 => waypoint = (-waypoint.1, waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (waypoint.1, -waypoint.0),
                _ => unreachable!(),
            },
            _ => {
                coords.0 += waypoint.0 * value;
                coords.1 += waypoint.1 * value;
            }
        }
    }
    coords.0.abs() + coords.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input());
        assert_eq!(25, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input());
        assert_eq!(286, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            F10
            N3
            F7
            R90
            F11
        "}
    }
}
