use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug)]
struct Group {
    answers: Vec<HashSet<char>>,
}

impl From<&str> for Group {
    fn from(line: &str) -> Self {
        Group {
            answers: line
                .split(" ")
                .map(|i| {
                    let mut form = HashSet::new();
                    for c in i.chars() {
                        form.insert(c);
                    }
                    form
                })
                .collect(),
        }
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Group> {
    input
        .replace('\n', " ")
        .split("  ")
        .map(|l| Group::from(l))
        .collect()
}

#[aoc(day6, part1)]
fn part1(form_groups: &[Group]) -> usize {
    form_groups.iter().fold(0, |acc, g| {
        let mut u = HashSet::new();
        for h in g.answers.iter() {
            u = u.union(&h).copied().collect();
        }
        acc + u.len()
    })
}

#[aoc(day6, part2)]
fn part2(form_groups: &[Group]) -> usize {
    form_groups.iter().fold(0, |acc, g| {
        let mut i = g.answers[0].clone();
        for h in g.answers.iter().skip(1) {
            i = i.intersection(&h).copied().collect();
        }
        acc + i.len()
    })
}
