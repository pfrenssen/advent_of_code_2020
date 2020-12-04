use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<u16>,
    hgt_unit: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    input
        .replace('\n', " ")
        .split("  ")
        .map(|l| Passport {
            byr: get_integer_value(l, "byr"),
            iyr: get_integer_value(l, "iyr"),
            eyr: get_integer_value(l, "eyr"),
            hgt: get_integer_value(l, "hgt"),
            hgt_unit: get_height_unit(l),
            hcl: get_string_value(l, "hcl"),
            ecl: get_string_value(l, "ecl"),
            pid: get_string_value(l, "pid"),
            cid: get_integer_value(l, "cid"),
        })
        .collect()
}

fn get_integer_value(l: &str, k: &str) -> Option<u16> {
    let regex = format!("{}{}", k, r":(\d+)");
    let re = Regex::new(regex.as_str()).unwrap();
    match re.captures(l) {
        Some(caps) => Some(caps[1].parse::<u16>().unwrap()),
        None => None,
    }
}

fn get_string_value(l: &str, k: &str) -> Option<String> {
    let regex = format!("{}{}", k, r":(\S+)");
    let re = Regex::new(regex.as_str()).unwrap();
    match re.captures(l) {
        Some(caps) => Some(caps[1].to_string()),
        None => None,
    }
}

fn get_height_unit(l: &str) -> Option<String> {
    let regex = r"hgt:\d+(\S+)";
    let re = Regex::new(regex).unwrap();
    match re.captures(l) {
        Some(caps) => Some(caps[1].to_string()),
        None => None,
    }
}

#[aoc(day4, part1)]
fn part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|&p| {
            p.byr.is_some()
                && p.iyr.is_some()
                && p.eyr.is_some()
                && p.hgt.is_some()
                && p.hcl.is_some()
                && p.ecl.is_some()
                && p.pid.is_some()
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &[Passport]) -> usize {
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();
    passports
        .iter()
        .filter(|&p| {
            p.byr.is_some()
                && p.byr.unwrap() >= 1920
                && p.byr.unwrap() <= 2002
                && p.iyr.is_some()
                && p.iyr.unwrap() >= 2010
                && p.iyr.unwrap() <= 2020
                && p.eyr.is_some()
                && p.eyr.unwrap() >= 2020
                && p.eyr.unwrap() <= 2030
                && p.hgt.is_some()
                && p.hgt_unit.is_some()
                && ((p.hgt_unit.as_ref().unwrap() == "cm"
                    && p.hgt.unwrap() >= 150
                    && p.hgt.unwrap() <= 193)
                    || (p.hgt_unit.as_ref().unwrap() == "in"
                        && p.hgt.unwrap() >= 59
                        && p.hgt.unwrap() <= 76))
                && p.hcl.is_some()
                && p.hcl.as_ref().unwrap().len() == 7
                && hcl_regex.is_match(p.hcl.as_ref().unwrap().as_str())
                && p.ecl.is_some()
                && ecl_regex.is_match(p.ecl.as_ref().unwrap().as_str())
                && p.pid.is_some()
                && pid_regex.is_match(p.pid.as_ref().unwrap().as_str())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};

        let lines = parse_input(input);
        assert_eq!(part1(&lines), 2);
    }
}
