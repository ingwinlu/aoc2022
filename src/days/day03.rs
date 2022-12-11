use std::{collections::HashMap, collections::HashSet};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn occurances(input: &str) -> HashMap<char, u64> {
    let mut map: HashMap<char, u64> = HashMap::new();
    for i in input.chars() {
        match map.get(&i) {
            None => map.insert(i, 1),
            Some(old_value) => map.insert(i, old_value + 1),
        };
    }
    map
}

fn calc_value(val: &char) -> u64 {
    let val_as_uint = if val.is_lowercase() {
        *val as u64 - 96
    } else {
        *val as u64 - 64 + 26
    };
    return val_as_uint;
}

fn solve_day1(input: &str) -> u64 {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let len = line.len();
        let half_1 = &line[..len / 2];
        let half_2 = &line[len / 2..];
        let set1: HashSet<char> = occurances(half_1).keys().cloned().collect();
        let set2: HashSet<char> = occurances(half_2).keys().cloned().collect();
        let mut diff = set1.intersection(&set2);
        let val = diff.next().unwrap();
        let val_as_uint = calc_value(val);
        // println!("{half_1} {half_2} {val} {val_as_uint}");

        total += val_as_uint;
    }
    return total;
}

fn solve_day2(input: &str) -> u64 {
    let mut total = 0;
    let line_sets: Vec<HashSet<char>> = input
        .lines()
        .map(occurances)
        .map(|occurance| occurance.keys().cloned().collect())
        .collect();

    for chunk in line_sets.chunks_exact(3) {
        if let [inter1, inter2, inter3] = &chunk {
            let intermediate: HashSet<_> = inter1.intersection(inter2).copied().collect();
            let intermediate: HashSet<_> = intermediate.intersection(inter3).collect();

            let val_as_uint = calc_value(intermediate.into_iter().next().unwrap());
            total += val_as_uint;
        } else {
            unreachable!();
        }
    }
    total
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day03.txt");
    let sol1: u64 = solve_day1(&input);
    let sol2: u64 = solve_day2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "};
        let result = solve_day1(input);
        assert_eq!(result, 157);
    }

    #[test]
    fn sample_day_1_per_line() {
        let input = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 16),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38),
            ("PmmdzqPrVvPwwTWBwg", 42),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22),
            ("ttgJtRGJQctTZtZT", 20),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", 19),
        ];
        for (line, expected) in input {
            assert_eq!(solve_day1(line), expected, "{line} has expected {expected}");
        }
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "};
        let result = solve_day2(input);
        assert_eq!(result, 70);
    }
}
