use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(expenses: &[i32]) -> i32 {
    let expense = expenses.iter().try_fold(0i32, |_, e: &i32,| {
        match expenses.contains(&(2020 - e)) {
            true => Err(*e),
            false => Ok(*e),
        }
    }).unwrap_err();
    expense * (2020 - expense)
}

#[aoc(day1, part2)]
fn part2(expenses: &[i32]) -> i32 {
    for i in expenses {
        let r = 2020 - i;
        match expenses.iter().try_fold(0i32, |_, e: &i32,| {
            match expenses.contains(&(r - e)) {
                true => Err(*e),
                false => Ok(*e),
            }
        }) {
            Ok(_) => {},
            Err(j) => return i * j * (2020 - i - j),
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
