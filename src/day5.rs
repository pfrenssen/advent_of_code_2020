use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct BoardingPass {
    row: String,
    seat: String,
}

impl From<&str> for BoardingPass {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"^([BF]{7})([LR]{3})$").unwrap();
        let caps = re.captures(line).unwrap();
        BoardingPass {
            row: caps[1].to_string(),
            seat: caps[2].to_string(),
        }
    }
}

impl BoardingPass {
    fn get_row_number(&self) -> u8 {
        u8::from_str_radix(self.row.replace("F", "0").replace("B", "1").as_str(), 2).unwrap()
    }
    fn get_seat_number(&self) -> u8 {
        u8::from_str_radix(self.seat.replace("L", "0").replace("R", "1").as_str(), 2).unwrap()
    }
    fn get_seat_id(&self) -> u16 {
        u16::from(self.get_row_number()) * 8u16 + u16::from(self.get_seat_number())
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|l| BoardingPass::from(l)).collect()
}

#[aoc(day5, part1)]
fn part1(passes: &[BoardingPass]) -> u16 {
    passes.iter().fold(0, |acc, p| match p.get_seat_id() > acc {
        true => p.get_seat_id(),
        false => acc,
    })
}

#[aoc(day5, part2)]
fn part2(passes: &[BoardingPass]) -> u16 {
    let mut passes: Vec<u16> = passes.iter().map(|p| p.get_seat_id()).collect();
    passes.sort();
    let mut previous = passes[0];
    passes.remove(0);

    for id in passes {
        let expected = previous + 1;
        if id != expected {
            return expected;
        }
        previous = id;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let test_cases = vec![
            ("FBFBBFFRLR", 44u8, 5u8, 357u16),
            ("BFFFBBFRRR", 70, 7, 567),
            ("FFFBBBFRRR", 14, 7, 119),
            ("BBFFBBFRLL", 102, 4, 820),
        ];
        for test_case in test_cases {
            let pass = BoardingPass::from(test_case.0);
            assert_eq!(pass.get_row_number(), test_case.1);
            assert_eq!(pass.get_seat_number(), test_case.2);
            assert_eq!(pass.get_seat_id(), test_case.3);
        }
    }
}
