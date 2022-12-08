use crate::{Solution, SolutionPair};
use std::fs;

fn beats(in1: &char, in2: &char) -> bool {
    if (in1 == &'A' && in2 == &'Z')
        || (in1 == &'B' && in2 == &'X')
        || (in1 == &'C' && in2 == &'Y')
        || (in1 == &'X' && in2 == &'C')
        || (in1 == &'Y' && in2 == &'A')
        || (in1 == &'Z' && in2 == &'B')
    {
        return true;
    }
    return false;
}

fn parse_line(line: &str) -> u64 {
    let mut chars = line.chars();
    let opponent = chars.next().unwrap();
    let player = chars.skip(1).next().unwrap();

    let mut score = player as u64 - 87;
    if beats(&player, &opponent) {
        //win
        score += 6;
    } else if beats(&opponent, &player) {
        // loss
    } else {
        // draw
        score += 3;
    }
    println!("{} {} {}", opponent, player, score);
    return score.into();
}

fn solve_day1(input: &str) -> u64 {
    let total_score = input.lines().map(parse_line).sum();
    return total_score;
}

fn parse_line_2(line: &str) -> u64 {
    let score: u64 = match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,

        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,

        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => unreachable!(),
    };
    return score.into();
}

fn solve_day2(input: &str) -> u64 {
    let total_score = input.lines().map(parse_line_2).sum();
    return total_score;
}

pub fn solve() -> SolutionPair {
    let input = fs::read_to_string("src/days/day02.txt").expect("File should not fail");
    let sol1: u64 = solve_day1(&input);
    let sol2: u64 = solve_day2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1_longer() {
        let input = indoc! {"
            C Y
            A Z
            B Y
            A Z
            A X
            A Z
            B Y
        "};
        let result = solve_day1(input);
        assert_eq!(result, 25);
    }
    #[test]
    fn sample_day_1_rock_scissors() {
        let input = indoc! {"
            A Z
        "};
        let result = solve_day1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
            A Y
            B X
            C Z
        "};
        let result = solve_day1(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            A Y
            B X
            C Z
        "};
        let result = solve_day2(input);
        assert_eq!(result, 12);
    }
}
