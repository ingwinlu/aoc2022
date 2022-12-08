use std::{collections::HashSet, fs};

use crate::{Solution, SolutionPair};

fn generic_solver(input: &str, window_size: usize) -> u64 {
    let windows = input.as_bytes().windows(window_size);
    for (offset, window) in windows.enumerate() {
        let mut uniq = HashSet::new();
        let is_uniq = window.into_iter().all(|x| uniq.insert(x));
        if is_uniq {
            return (offset + window_size) as u64;
        }
    }
    unreachable!()
}
fn solve_day1(input: &str) -> u64 {
    generic_solver(input, 4)
}

fn solve_day2(input: &str) -> u64 {
    generic_solver(input, 14)
}
pub fn solve() -> SolutionPair {
    let input = fs::read_to_string("src/days/day06.txt").expect("File should not fail");
    let sol1 = solve_day1(&input);
    let sol2 = solve_day2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_day_1() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input, expected) in inputs {
            let result = solve_day1(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn sample_day_2() {
        let inputs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, expected) in inputs {
            let result = solve_day2(input);
            assert_eq!(result, expected);
        }
    }
}
