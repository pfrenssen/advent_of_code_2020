use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Line {
    trees: Vec<usize>,
    width: usize,
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let v: Vec<_> = l.match_indices('#').map(|(i, _)| i).collect();
            Line {
                trees: v,
                width: l.len(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(lines: &[Line]) -> i32 {
    let mut x = 0usize;
    lines.iter().skip(1).fold(0, |acc, l: &Line| {
        x = (x + 3) % l.width;
        match l.trees.contains(&x) {
            true => acc + 1,
            false => acc,
        }
    })
}

#[aoc(day3, part2)]
fn part2(lines: &[Line]) -> i64 {
    let trial_runs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    trial_runs.iter().fold(1, |mul, (x, y)| {
        let mut pos = 0usize;
        lines.iter().skip(*y).step_by(*y).fold(0, |acc, l: &Line| {
            pos = (pos + x) % l.width;
            match l.trees.contains(&pos) {
                true => acc + 1,
                false => acc,
            }
        }) * mul
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[
                Line {
                    width: 11,
                    trees: vec!(2, 3)
                },
                Line {
                    width: 11,
                    trees: vec!(0, 4, 8)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 6, 9)
                },
                Line {
                    width: 11,
                    trees: vec!(2, 4, 8, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 5, 6, 9)
                },
                Line {
                    width: 11,
                    trees: vec!(2, 4, 5)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 3, 5, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(0, 2, 3, 7)
                },
                Line {
                    width: 11,
                    trees: vec!(0, 4, 5, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 4, 8, 10)
                },
            ]),
            7
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[
                Line {
                    width: 11,
                    trees: vec!(2, 3)
                },
                Line {
                    width: 11,
                    trees: vec!(0, 4, 8)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 6, 9)
                },
                Line {
                    width: 11,
                    trees: vec!(2, 4, 8, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 5, 6, 9)
                },
                Line {
                    width: 11,
                    trees: vec!(2, 4, 5)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 3, 5, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(0, 2, 3, 7)
                },
                Line {
                    width: 11,
                    trees: vec!(0, 4, 5, 10)
                },
                Line {
                    width: 11,
                    trees: vec!(1, 4, 8, 10)
                },
            ]),
            336
        );
    }
}
