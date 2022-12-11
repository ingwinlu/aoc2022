use crate::{Solution, SolutionPair};
use std::collections::HashSet;
///////////////////////////////////////////////////////////////////////////////

fn parse_range(range: &str) -> HashSet<u64> {
    let mut range_iter = range.split("-");
    let start: u64 = range_iter.next().unwrap().parse().unwrap();
    let stop: u64 = range_iter.next().unwrap().parse().unwrap();

    let s = HashSet::from_iter(start..=stop);
    //println!("{s:?}");
    s
}

fn parse_line(line: &str) -> u64 {
    let mut pair_iter = line.split(",").map(parse_range);
    let first = pair_iter.next().unwrap();
    let second = pair_iter.next().unwrap();

    if first.is_subset(&second) || second.is_subset(&first) {
        1
    } else {
        0
    }
}

fn solve_day1(input: &str) -> u64 {
    input.lines().map(parse_line).sum()
}

fn parse_line_2(line: &str) -> u64 {
    let mut pair_iter = line.split(",").map(parse_range);
    let first = pair_iter.next().unwrap();
    let second = pair_iter.next().unwrap();

    if first.is_disjoint(&second) {
        0
    } else {
        1
    }
}

fn solve_day2(input: &str) -> u64 {
    input.lines().map(parse_line_2).sum()
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day04.txt");
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
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "};
        let result = solve_day1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "};
        let result = solve_day2(input);
        assert_eq!(result, 4);
    }
}
