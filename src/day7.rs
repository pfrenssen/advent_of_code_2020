use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, Vec<(u8, String)>> {
    input
        .lines()
        .map(|l| {
            let re = Regex::new(r"^(\w+ \w+) bags contain (.*)\.$").unwrap();
            let caps = re.captures(l).unwrap();
            let key = caps[1].to_string();
            let content: Vec<(u8, String)> = match &caps[2] {
                "no other bags" => vec![],
                _ => caps[2]
                    .split(", ")
                    .map(|c| {
                        let re = Regex::new(r"^(\d+) (\w+ \w+)").unwrap();
                        let caps = re.captures(c).unwrap();
                        (caps[1].parse::<u8>().unwrap(), caps[2].to_string())
                    })
                    .collect(),
            };
            (key, content)
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(rules: &HashMap<String, Vec<(u8, String)>>) -> usize {
    let mut rules = rules.clone();
    rules.remove("shiny gold");
    rules
        .iter()
        .filter(|(k, _)| can_contain_shiny_gold(k, &rules))
        .count()
}

fn can_contain_shiny_gold(k: &str, rules: &HashMap<String, Vec<(u8, String)>>) -> bool {
    let contents: Vec<&str> = rules
        .get(k)
        .unwrap()
        .iter()
        .map(|(_, v)| v.as_str())
        .collect();
    if contents.is_empty() {
        return false;
    }
    if contents.contains(&"shiny gold") {
        return true;
    }
    contents
        .iter()
        .fold(false, |acc, c| acc || can_contain_shiny_gold(c, rules))
}

#[aoc(day7, part2)]
fn part2(rules: &HashMap<String, Vec<(u8, String)>>) -> usize {
    get_bag_count("shiny gold", rules)
}

fn get_bag_count(k: &str, rules: &HashMap<String, Vec<(u8, String)>>) -> usize {
    let contents = rules.get(k).unwrap();
    let count = rules
        .get(k)
        .unwrap()
        .iter()
        .fold(0, |acc, (c, _)| acc + usize::from(*c));
    if contents.is_empty() {
        return 0;
    }
    contents.iter().fold(count, |acc, (c, k)| {
        acc + usize::from(*c) * get_bag_count(k, rules)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let input = get_test_input();
        let result = parse_input(input);
        assert_eq!(result.len(), 9);
        assert_eq!(
            result.get(&"light red".to_string()).unwrap(),
            &vec!(
                (1u8, "bright white".to_string()),
                (2, "muted yellow".to_string())
            )
        );
        assert_eq!(
            result.get(&"bright white".to_string()).unwrap(),
            &vec!((1u8, "shiny gold".to_string()))
        );
        assert_eq!(result.get(&"faded blue".to_string()).unwrap(), &vec!());
    }

    #[test]
    fn test_part1() {
        let input = parse_input(get_test_input());
        assert_eq!(4, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(get_test_input());
        assert_eq!(32, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "}
    }
}
