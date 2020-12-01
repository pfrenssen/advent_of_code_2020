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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }
}
