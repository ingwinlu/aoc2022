use std::collections::HashSet;

use crate::{Solution, SolutionPair};

type Coords = (i32, i32);

fn dist(c1: &Coords, c2: &Coords) -> u32 {
    let x_diff = c1.0.abs_diff(c2.0);
    let y_diff = c1.1.abs_diff(c2.1);
    x_diff.max(y_diff)
}

fn solve_day1(input: &str) -> u64 {
    let instructions = input.lines();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = vec![tail];
    for inst in instructions {
        let to_move = inst[2..].parse::<i32>().unwrap();

        for _ in 0..to_move {
            // println!("{}: {:?}, {:?}, ", inst, head, tail);
            // move head
            match inst {
                _ if inst.starts_with("R") => head.0 += 1,
                _ if inst.starts_with("U") => head.1 += 1,
                _ if inst.starts_with("L") => head.0 -= 1,
                _ if inst.starts_with("D") => head.1 -= 1,
                _ => unreachable!(),
            }

            // move tail if distance is large enough
            let dista = dist(&head, &tail);
            if dista > 2 {
                panic!();
            }
            if dista > 1 {
                if head.0 == tail.0 {
                    // move tail in y direction
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    // move tail in x direction
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else if head.0.abs_diff(tail.0) == 2 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1.abs_diff(tail.1) == 2 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else {
                    unreachable!("{head:?} {tail:?} {dista}");
                }
                tail_positions.push(tail);
            }
        }
    }
    let tail_set: HashSet<Coords> = HashSet::from_iter(tail_positions.iter().cloned());
    // println!("{tail_set:?}");
    tail_set.len() as u64
}

fn solve_day2(input: &str) -> u64 {
    0
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day09.txt");
    let sol1 = solve_day1(&input);
    let sol2 = solve_day2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        let result = solve_day2(&input);
        assert_eq!(result, 1);
    }
}
