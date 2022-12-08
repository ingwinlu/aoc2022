use crate::{Solution, SolutionPair};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref MOVE_RE: Regex =
        Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
}

fn solve_day1(input: &str) -> String {
    let (start, instructions) = input.split_once("\n\n").unwrap();

    let mut parse_start_setup_iter = start.lines().rev();
    let last_line = parse_start_setup_iter.next().unwrap();
    let mut stacks: Vec<Vec<char>> = last_line.split_whitespace().map(|_id| Vec::new()).collect();

    for line in parse_start_setup_iter {
        for (i, stack) in stacks.iter_mut().enumerate() {
            if let Some(cra) = line.chars().nth(1 + i * 4) {
                if cra.is_ascii_uppercase() {
                    stack.push(cra);
                }
            };
        }
    }

    for instruction in instructions.lines() {
        let instr_parsed = MOVE_RE
            .captures(instruction)
            .expect(&format!("{instruction} could not be parsed"));
        let amount: usize = instr_parsed
            .name("amount")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let from: usize = instr_parsed.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = instr_parsed.name("to").unwrap().as_str().parse().unwrap();
        // println!("{stacks:?} {amount} {from} {to}");

        let from_stack = stacks.get_mut(from - 1).unwrap();

        let start = from_stack.len() - amount;
        let mut to_move = from_stack.split_off(start);
        to_move.reverse();

        //println!("{from_stack:?} {} {start} {to_move:?}", from_stack.len());

        let to_stack = stacks.get_mut(to - 1).unwrap();
        to_stack.append(&mut to_move);
    }

    let result: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect();
    // println!("{stacks:?}");
    result
}

fn solve_day2(input: &str) -> String {
    let (start, instructions) = input.split_once("\n\n").unwrap();

    let mut parse_start_setup_iter = start.lines().rev();
    let last_line = parse_start_setup_iter.next().unwrap();
    let mut stacks: Vec<Vec<char>> = last_line.split_whitespace().map(|_id| Vec::new()).collect();

    for line in parse_start_setup_iter {
        for (i, stack) in stacks.iter_mut().enumerate() {
            if let Some(cra) = line.chars().nth(1 + i * 4) {
                if cra.is_ascii_uppercase() {
                    stack.push(cra);
                }
            };
        }
    }

    for instruction in instructions.lines() {
        let instr_parsed = MOVE_RE
            .captures(instruction)
            .expect(&format!("{instruction} could not be parsed"));
        let amount: usize = instr_parsed
            .name("amount")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let from: usize = instr_parsed.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = instr_parsed.name("to").unwrap().as_str().parse().unwrap();
        // println!("{stacks:?} {amount} {from} {to}");

        let from_stack = stacks.get_mut(from - 1).unwrap();

        let start = from_stack.len() - amount;
        let mut to_move = from_stack.split_off(start);
        // to_move.reverse();

        //println!("{from_stack:?} {} {start} {to_move:?}", from_stack.len());

        let to_stack = stacks.get_mut(to - 1).unwrap();
        to_stack.append(&mut to_move);
    }

    let result: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect();
    // println!("{stacks:?}");
    result
}
pub fn solve() -> SolutionPair {
    let input = fs::read_to_string("src/days/day05.txt").expect("File should not fail");
    let sol1: String = solve_day1(&input);
    let sol2: String = solve_day2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
                [D]
            [N] [C]
            [Z] [M] [P]
            1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "};
        let result = solve_day1(input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
                [D]
            [N] [C]
            [Z] [M] [P]
            1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "};
        let result = solve_day2(input);
        assert_eq!(result, "MCD");
    }
}
