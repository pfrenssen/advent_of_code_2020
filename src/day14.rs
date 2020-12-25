use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use Instruction::{BitMask, MemWrite};

#[derive(Debug, PartialEq)]
enum Instruction {
    BitMask(String),
    MemWrite(u64, u64),
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Instruction> {
    let re_mask = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            if re_mask.is_match(l) {
                let joana = re_mask.captures(l).unwrap();
                return BitMask(joana[1].to_string());
            }
            let joana = re_mem.captures(l).unwrap();
            MemWrite(joana[1].parse().unwrap(), joana[2].parse().unwrap())
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(instructions: &Vec<Instruction>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();
    for instruction in instructions {
        match instruction {
            BitMask(b) => mask = b.clone(),
            MemWrite(a, v) => {
                let or = u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap();
                let and = u64::from_str_radix(mask.replace("X", "1").as_str(), 2).unwrap();
                mem.insert(*a, (v | or) & and);
            }
        }
    }
    mem.iter().map(|(_, v)| *v).sum()
}

#[aoc(day14, part2)]
fn part2(instructions: &Vec<Instruction>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();
    for instruction in instructions {
        match instruction {
            BitMask(b) => mask = b.clone(),
            MemWrite(a, v) => {
                let mut addresses = vec![];
                addresses.push(mask.clone());

                'a: loop {
                    for i in 0..addresses.len() {
                        let value = addresses[i].clone();
                        if !value.contains("X") {
                            continue;
                        }
                        addresses.remove(i);
                        addresses.push(value.replacen("X", "0", 1));
                        addresses.push(value.replacen("X", "1", 1));
                        continue 'a;
                    }
                    break;
                }

                let a =
                    a & u64::from_str_radix(mask.replace("0", "1").replace("X", "0").as_str(), 2)
                        .unwrap();
                //let a = a & amask;
                for address in addresses {
                    let address = u64::from_str_radix(address.as_str(), 2).unwrap() | a;
                    mem.insert(address, *v);
                }
            }
        }
    }
    mem.iter().map(|(_, v)| *v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day14::Instruction::{BitMask, MemWrite};
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            BitMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
            MemWrite(8, 11),
            MemWrite(7, 101),
            MemWrite(8, 0),
        ];
        assert_eq!(expected, parse_input(get_test_input_part1()));

        let expected = vec![
            BitMask("000000000000000000000000000000X1001X".to_string()),
            MemWrite(42, 100),
            BitMask("00000000000000000000000000000000X0XX".to_string()),
            MemWrite(26, 1),
        ];
        assert_eq!(expected, parse_input(get_test_input_part2()));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input_part1());
        assert_eq!(165, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input_part2());
        assert_eq!(208, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        indoc! {"
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        "}
    }
}
