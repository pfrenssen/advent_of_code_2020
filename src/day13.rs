use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13, part1)]
fn parse_input_1(input: &str) -> (usize, Vec<usize>) {
    let lines: Vec<&str> = input.lines().collect();
    let values = lines[1]
        .split(',')
        .filter(|v| v.parse::<usize>().is_ok())
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    (lines[0].parse().unwrap(), values)
}

#[aoc(day13, part1)]
fn part1(bus_info: &(usize, Vec<usize>)) -> usize {
    let timestamp = bus_info.0;
    let bus_map: Vec<(usize, usize)> = bus_info
        .1
        .iter()
        .map(|i| (*i, i - (timestamp % i)))
        .collect();
    let (id, time) = bus_map.iter().fold(bus_map[0], |acc, (id, time)| {
        if *time < acc.1 {
            return (*id, *time);
        }
        return acc;
    });

    id * time
}

#[aoc_generator(day13, part2)]
fn parse_input_2(input: &str) -> Vec<Option<u64>> {
    let lines: Vec<&str> = input.lines().collect();
    lines[1].split(',').map(|v| v.parse::<u64>().ok()).collect()
}

#[aoc(day13, part2)]
fn part2(bus_info: &Vec<Option<u64>>) -> u64 {
    let mut time = 0;
    let mut interval = bus_info[0].unwrap();
    'outer: loop {
        let mut delta = 0;
        let mut new_interval = 1;
        for bus in bus_info {
            match bus {
                Some(id) => {
                    new_interval *= id;
                    if (delta + time) % id != 0 {
                        time += interval;
                        continue 'outer;
                    }

                    if new_interval > interval {
                        interval = new_interval;
                    }
                }
                None => {}
            }
            delta += 1;
        }
        break;
    }
    time
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_1() {
        let expected: (usize, Vec<usize>) = (939, vec![7, 13, 59, 31, 19]);
        assert_eq!(expected, parse_input_1(get_test_input()));
    }

    #[test]
    fn test_part1() {
        let input = parse_input_1(get_test_input());
        assert_eq!(295, part1(&input));
    }

    #[test]
    fn test_parse_input_2() {
        let expected: Vec<Option<u64>> = vec![
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19),
        ];
        assert_eq!(expected, parse_input_2(get_test_input()));
    }

    #[test]
    fn test_part2() {
        let test_cases = vec![
            (
                indoc! {"
                    939
                    7,13,x,x,59,x,31,19
                "},
                1068781,
            ),
            (
                indoc! {"
                    939
                    17,x,13,19
                "},
                3417,
            ),
            (
                indoc! {"
                    939
                    67,7,59,61
                "},
                754018,
            ),
            (
                indoc! {"
                    939
                    67,x,7,59,61
                "},
                779210,
            ),
            (
                indoc! {"
                    939
                    67,7,x,59,61
                "},
                1261476,
            ),
            (
                indoc! {"
                    939
                    1789,37,47,1889
                "},
                1202161486,
            ),
        ];
        for (input, expected) in test_cases {
            assert_eq!(expected, part2(&parse_input_2(input)))
        }
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            939
            7,13,x,x,59,x,31,19
        "}
    }
}
