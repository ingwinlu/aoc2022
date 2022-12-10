use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum MODE {
    IDLE,
    ADDX(i32, i32),
}

fn solve_day1(input: &str) -> u64 {
    let mut instructions = input.lines();
    let mut current_mode = MODE::IDLE;
    let mut x = 1;
    let mut signals = Vec::with_capacity(5);
    for cycle in 1..=220 {
        // println!("Cycle {}, x {},  MODE {:?}", cycle, x, current_mode);
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            let signal_strength = x * cycle;
            // println!("** {signal_strength} **");
            signals.push(signal_strength)
        }
        match current_mode {
            MODE::IDLE => {
                let maybe_instruction = instructions.next();
                // println!("-> {maybe_instruction:?}");
                match maybe_instruction {
                    Some("noop") => {}
                    Some(instruction) if instruction.starts_with("addx ") => {
                        let to_add = instruction[4..].trim().parse::<i32>().unwrap();
                        current_mode = MODE::ADDX(cycle + 1, to_add);
                    }
                    Some(unknown_instruction) => {
                        panic!("Unknown instruction, {unknown_instruction}")
                    }
                    None => {
                        println!("Instructions ended");
                        break;
                    }
                }
            }
            MODE::ADDX(cycle_to_finish, to_add) => {
                if cycle == cycle_to_finish {
                    x += to_add;
                    current_mode = MODE::IDLE;
                }
            }
        }
    }
    // println!("{signals:?}");
    let s: i32 = signals.iter().sum();
    s.try_into().unwrap()
}

fn solve_day2(input: &str) -> String {
    let mut instructions = input.lines();
    let mut current_mode = MODE::IDLE;
    let mut x = 1;
    let mut solution = String::with_capacity(6 * 40);
    for cycle in 1..=240 {
        // println!("Cycle {}, x {},  MODE {:?}", cycle, x, current_mode);
        let crt_pos = (cycle - 1) % 40;
        // sprite goes from x-1 to x + 1
        // if crt_pos is on sprite -> draw #
        if (x - 1) <= crt_pos && (x + 1) >= crt_pos {
            solution.push('#');
        } else {
            solution.push('.');
        }
        if crt_pos == 39 {
            solution.push('\n');
        }

        match current_mode {
            MODE::IDLE => {
                let maybe_instruction = instructions.next();
                // println!("-> {maybe_instruction:?}");
                match maybe_instruction {
                    Some("noop") => {}
                    Some(instruction) if instruction.starts_with("addx ") => {
                        let to_add = instruction[4..].trim().parse::<i32>().unwrap();
                        current_mode = MODE::ADDX(cycle + 1, to_add);
                    }
                    Some(unknown_instruction) => {
                        panic!("Unknown instruction, {unknown_instruction}")
                    }
                    None => {
                        println!("Instructions ended");
                        break;
                    }
                }
            }
            MODE::ADDX(cycle_to_finish, to_add) => {
                if cycle == cycle_to_finish {
                    x += to_add;
                    current_mode = MODE::IDLE;
                }
            }
        }
    }
    solution
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day10.txt");
    let sol1 = solve_day1(&input);
    let sol2 = solve_day2(&input);

    (
        Solution::from(sol1),
        Solution::from("\n".to_owned() + &sol2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
            noop
            addx 3
            addx -5
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn sample_day_1_longer() {
        let input = indoc! {"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
        "};
        let result = solve_day2(&input);
        assert_eq!(
            result,
            indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "}
        );
    }
}
