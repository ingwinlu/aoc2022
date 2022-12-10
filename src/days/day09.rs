use std::collections::HashSet;

use crate::{Solution, SolutionPair};

type Coords = (i32, i32);

fn hamil_dist(c1: &Coords, c2: &Coords) -> u32 {
    let x_diff = c1.0.abs_diff(c2.0);
    let y_diff = c1.1.abs_diff(c2.1);
    x_diff.max(y_diff)
}

fn calculate_tail_pos(input: &str, tail_count: usize) -> u64 {
    let instructions = input.lines();
    let mut knots: Vec<Coords> = [(0, 0)].into_iter().cycle().take(1 + tail_count).collect();
    let last_knot_pos = knots.len() - 1;
    let mut last_knot_positions = vec![knots[last_knot_pos]];
    for inst in instructions {
        let to_move = inst[2..].parse::<i32>().unwrap();

        for _ in 0..to_move {
            // println!("{}: {:?}, {:?}, ", inst, head, tail);
            // move head

            match inst {
                _ if inst.starts_with("R") => knots[0].0 += 1,
                _ if inst.starts_with("U") => knots[0].1 += 1,
                _ if inst.starts_with("L") => knots[0].0 -= 1,
                _ if inst.starts_with("D") => knots[0].1 -= 1,
                _ => unreachable!(),
            }

            // move tails
            for i in 1..knots.len() {
                let head = knots[i - 1];
                let mut tail = &mut knots[i];
                //if distance is large enough
                let dista = hamil_dist(&head, &tail);
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
                }
            }

            last_knot_positions.push(knots[last_knot_pos]);
        }
    }
    let tail_set: HashSet<Coords> = HashSet::from_iter(last_knot_positions.iter().cloned());
    // println!("{tail_set:?}");
    tail_set.len() as u64
}

fn solve_day1(input: &str) -> u64 {
    calculate_tail_pos(&input, 1)
}

fn solve_day2(input: &str) -> u64 {
    calculate_tail_pos(&input, 9)
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

    #[test]
    fn sample_day_2_larger() {
        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        let result = solve_day2(&input);
        assert_eq!(result, 36);
    }
}
