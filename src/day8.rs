use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashSet;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<(String, isize)> {
    let re = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();
    input
        .lines()
        .map(|l| {
            let c = re.captures(l).unwrap();
            (c[1].to_string(), c[2].parse::<isize>().unwrap())
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(code: &Vec<(String, isize)>) -> isize {
    bootstrap(code).unwrap_err()
}

#[aoc(day8, part2)]
fn part2(code: &Vec<(String, isize)>) -> isize {
    for i in 0..code.len() {
        let mut mutated = code.clone();
        let instruction = mutated.get_mut(i).unwrap();

        match instruction.0.as_str() {
            "acc" => continue,
            "nop" => instruction.0 = "jmp".to_string(),
            _ => instruction.0 = "nop".to_string(),
        }

        match bootstrap(&mutated) {
            Ok(r) => return r,
            Err(_) => continue,
        }
    }
    unreachable!()
}

fn bootstrap(code: &Vec<(String, isize)>) -> Result<isize, isize> {
    let mut executed: HashSet<usize> = HashSet::new();
    let mut pos = 0isize;
    let mut acc = 0;

    loop {
        if executed.contains(&(pos as usize)) {
            return Err(acc);
        }
        executed.insert(pos as usize);

        let instr = match code.get(pos as usize) {
            Some(instr) => instr,
            None => return Ok(acc),
        };
        match instr.0.as_str() {
            "jmp" => pos = pos + instr.1 - 1,
            "acc" => acc = acc + instr.1,
            _ => {}
        }
        pos = pos + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            ("nop".to_string(), 0),
            ("acc".to_string(), 1),
            ("jmp".to_string(), 4),
            ("acc".to_string(), 3),
            ("jmp".to_string(), -3),
            ("acc".to_string(), -99),
            ("acc".to_string(), 1),
            ("jmp".to_string(), -4),
            ("acc".to_string(), 6),
        ];
        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input());
        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input());
        assert_eq!(8, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "}
    }
}
