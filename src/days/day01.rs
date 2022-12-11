use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn parse(input: &str) -> Vec<u64> {
    let mut elf_calory_map: Vec<u64> = Vec::new();
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            elf_calory_map.push(current);
            current = 0;
        } else {
            current += line.parse::<u64>().unwrap();
        }
    }
    elf_calory_map.push(current);
    return elf_calory_map;
}

pub fn solve_day1(input: &str) -> u64 {
    let elf_calory_map = parse(input);

    let max = *elf_calory_map.iter().max().expect("Should return max");
    return max;
}

pub fn solve_day2(input: &str) -> u64 {
    let mut elf_calory_map = parse(input);

    elf_calory_map.sort();
    let top3 = elf_calory_map.iter().rev().take(3).sum();
    return top3;
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day01.txt");

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
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "};
        let result = solve_day1(input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "};
        let result = solve_day2(input);
        assert_eq!(result, 45000);
    }
}
