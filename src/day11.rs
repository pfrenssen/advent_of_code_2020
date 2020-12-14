use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> HashMap<(usize, usize), Option<bool>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut map = HashMap::new();
    for y in 0..lines.len() {
        let line = lines[y];
        let mut x = 0;
        for loc in line.chars() {
            map.insert(
                (x, y),
                match loc {
                    'L' => Some(false),
                    '#' => Some(true),
                    _ => None,
                },
            );
            x += 1;
        }
    }
    map
}

#[aoc(day11, part1)]
fn part1(map: &HashMap<(usize, usize), Option<bool>>) -> usize {
    let mut map = map.clone();
    loop {
        let updated = update_adjacent_seats(&map);
        if updated.eq(&map) {
            break;
        }
        map = updated;
    }
    map.iter()
        .filter(|(_, value)| **value == Some(true))
        .count()
}

fn update_adjacent_seats(
    map: &HashMap<(usize, usize), Option<bool>>,
) -> HashMap<(usize, usize), Option<bool>> {
    let mut updated_map = HashMap::new();
    map.iter().for_each(|(coords, _)| {
        updated_map.insert(*coords, update_adjacent_seat(map, coords.0, coords.1));
    });
    updated_map
}

fn update_adjacent_seat(
    map: &HashMap<(usize, usize), Option<bool>>,
    x: usize,
    y: usize,
) -> Option<bool> {
    match map.get(&(x, y)).unwrap() {
        Some(false) => match count_adjacent_occupied_seats(map, x as isize, y as isize) {
            0 => Some(true),
            _ => Some(false),
        },
        Some(true) => match count_adjacent_occupied_seats(map, x as isize, y as isize) {
            val if val >= 4 => Some(false),
            _ => Some(true),
        },
        None => None,
    }
}

fn count_adjacent_occupied_seats(
    map: &HashMap<(usize, usize), Option<bool>>,
    x: isize,
    y: isize,
) -> u8 {
    let mut count = 0;
    for xx in x - 1..=x + 1 {
        for yy in y - 1..=y + 1 {
            if xx < 0 || yy < 0 || (xx == x && yy == y) {
                continue;
            }
            if map.get(&(xx as usize, yy as usize)).unwrap_or(&None) == &Some(true) {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day11, part2)]
fn part2(map: &HashMap<(usize, usize), Option<bool>>) -> usize {
    let mut map = map.clone();
    loop {
        let updated = update_visible_seats(&map);
        if updated.eq(&map) {
            break;
        }
        map = updated;
    }
    map.iter()
        .filter(|(_, value)| **value == Some(true))
        .count()
}

fn update_visible_seats(
    map: &HashMap<(usize, usize), Option<bool>>,
) -> HashMap<(usize, usize), Option<bool>> {
    let mut updated_map = HashMap::new();
    map.iter().for_each(|(coords, _)| {
        updated_map.insert(*coords, update_visible_seat(map, coords.0, coords.1));
    });
    updated_map
}

fn update_visible_seat(
    map: &HashMap<(usize, usize), Option<bool>>,
    x: usize,
    y: usize,
) -> Option<bool> {
    match map.get(&(x, y)).unwrap() {
        Some(false) => match count_visible_occupied_seats(map, x as isize, y as isize) {
            0 => Some(true),
            _ => Some(false),
        },
        Some(true) => match count_visible_occupied_seats(map, x as isize, y as isize) {
            val if val >= 5 => Some(false),
            _ => Some(true),
        },
        None => None,
    }
}

fn count_visible_occupied_seats(
    map: &HashMap<(usize, usize), Option<bool>>,
    xpos: isize,
    ypos: isize,
) -> u8 {
    let mut count = 0;

    for xdelta in -1..=1 {
        for ydelta in -1..=1 {
            if xdelta == 0 && ydelta == 0 {
                continue;
            }
            if occupied_seat_visible(map, xpos, ypos, xdelta, ydelta) {
                count += 1;
            }
        }
    }
    count
}

fn occupied_seat_visible(
    map: &HashMap<(usize, usize), Option<bool>>,
    xpos: isize,
    ypos: isize,
    xdelta: isize,
    ydelta: isize,
) -> bool {
    let xpos = xpos + xdelta;
    let ypos = ypos + ydelta;
    if xpos < 0 || ypos < 0 {
        return false;
    }
    match map.get(&(xpos as usize, ypos as usize)) {
        Some(value) => match value {
            Some(occupied) => *occupied,
            None => occupied_seat_visible(map, xpos, ypos, xdelta, ydelta),
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(get_test_input());
        let expected: Vec<(usize, usize, Option<bool>)> = vec![
            (0, 0, Some(false)),
            (1, 0, None),
            (2, 0, Some(false)),
            (3, 0, Some(false)),
            (4, 0, None),
            (5, 0, Some(false)),
            (6, 0, Some(false)),
            (7, 0, None),
            (8, 0, Some(false)),
            (9, 0, Some(false)),
            (0, 1, Some(false)),
            (1, 1, Some(false)),
            (2, 1, Some(false)),
            (3, 1, Some(false)),
            (4, 1, Some(false)),
            (5, 1, Some(false)),
            (6, 1, Some(false)),
            (7, 1, None),
            (8, 1, Some(false)),
            (9, 1, Some(false)),
            (0, 2, Some(false)),
            (1, 2, None),
            (2, 2, Some(false)),
            (3, 2, None),
            (4, 2, Some(false)),
            (5, 2, None),
            (6, 2, None),
            (7, 2, Some(false)),
            (8, 2, None),
            (9, 2, None),
            (0, 3, Some(false)),
            (1, 3, Some(false)),
            (2, 3, Some(false)),
            (3, 3, Some(false)),
            (4, 3, None),
            (5, 3, Some(false)),
            (6, 3, Some(false)),
            (7, 3, None),
            (8, 3, Some(false)),
            (9, 3, Some(false)),
            (0, 4, Some(false)),
            (1, 4, None),
            (2, 4, Some(false)),
            (3, 4, Some(false)),
            (4, 4, None),
            (5, 4, Some(false)),
            (6, 4, Some(false)),
            (7, 4, None),
            (8, 4, Some(false)),
            (9, 4, Some(false)),
            (0, 5, Some(false)),
            (1, 5, None),
            (2, 5, Some(false)),
            (3, 5, Some(false)),
            (4, 5, Some(false)),
            (5, 5, Some(false)),
            (6, 5, Some(false)),
            (7, 5, None),
            (8, 5, Some(false)),
            (9, 5, Some(false)),
            (0, 6, None),
            (1, 6, None),
            (2, 6, Some(false)),
            (3, 6, None),
            (4, 6, Some(false)),
            (5, 6, None),
            (6, 6, None),
            (7, 6, None),
            (8, 6, None),
            (9, 6, None),
            (0, 7, Some(false)),
            (1, 7, Some(false)),
            (2, 7, Some(false)),
            (3, 7, Some(false)),
            (4, 7, Some(false)),
            (5, 7, Some(false)),
            (6, 7, Some(false)),
            (7, 7, Some(false)),
            (8, 7, Some(false)),
            (9, 7, Some(false)),
            (0, 8, Some(false)),
            (1, 8, None),
            (2, 8, Some(false)),
            (3, 8, Some(false)),
            (4, 8, Some(false)),
            (5, 8, Some(false)),
            (6, 8, Some(false)),
            (7, 8, Some(false)),
            (8, 8, None),
            (9, 8, Some(false)),
            (0, 9, Some(false)),
            (1, 9, None),
            (2, 9, Some(false)),
            (3, 9, Some(false)),
            (4, 9, Some(false)),
            (5, 9, Some(false)),
            (6, 9, Some(false)),
            (7, 9, None),
            (8, 9, Some(false)),
            (9, 9, Some(false)),
        ];
        for test_case in expected {
            let actual = parsed_input.get(&(test_case.0, test_case.1)).unwrap();
            assert_eq!(
                actual, &test_case.2,
                "Testing coordinates {},{}",
                test_case.0, test_case.1
            );
        }
    }

    #[test]
    fn test_update_seats() {
        let mut initial = parse_input(indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "});
        let results = vec![
            indoc! {"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
            "},
            indoc! {"
                #.LL.L#.##
                #LLLLLL.L#
                L.L.L..L..
                #LLL.LL.L#
                #.LL.LL.LL
                #.LLLL#.##
                ..L.L.....
                #LLLLLLLL#
                #.LLLLLL.L
                #.#LLLL.##
            "},
            indoc! {"
                #.##.L#.##
                #L###LL.L#
                L.#.#..#..
                #L##.##.L#
                #.##.LL.LL
                #.###L#.##
                ..#.#.....
                #L######L#
                #.LL###L.L
                #.#L###.##
            "},
            indoc! {"
                #.#L.L#.##
                #LLL#LL.L#
                L.L.L..#..
                #LLL.##.L#
                #.LL.LL.LL
                #.LL#L#.##
                ..L.L.....
                #L#LLLL#L#
                #.LLLLLL.L
                #.#L#L#.##
            "},
            indoc! {"
                #.#L.L#.##
                #LLL#LL.L#
                L.#.L..#..
                #L##.##.L#
                #.#L.LL.LL
                #.#L#L#.##
                ..L.L.....
                #L#L##L#L#
                #.LLLLLL.L
                #.#L#L#.##
            "},
            indoc! {"
                #.#L.L#.##
                #LLL#LL.L#
                L.#.L..#..
                #L##.##.L#
                #.#L.LL.LL
                #.#L#L#.##
                ..L.L.....
                #L#L##L#L#
                #.LLLLLL.L
                #.#L#L#.##
            "},
        ];
        for result in results {
            let actual = update_adjacent_seats(&initial);
            assert_eq!(actual, parse_input(result));
            initial = actual;
        }
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input());
        assert_eq!(37, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input());
        assert_eq!(26, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "}
    }
}
