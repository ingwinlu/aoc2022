use std::collections::HashMap;

use crate::{Solution, SolutionPair};

type Coordinates = (i64, i64); // x, y

fn solve_day1(input: &str) -> u64 {
    let mut map = parse(input);
    loop {
        // spawn sand
        println!("spawn");
        let mut current = (500, 0);
        let mut could_place = false;

        // let sand flow for some ticks
        for _tick in 0..1_000 {
            // check if able to move
            let possibilities = [(0, 1), (-1, 1), (1, 1)];
            let mut found_possiblity = false;
            for possibility in possibilities {
                let next_pos = (current.0 + possibility.0, current.1 + possibility.1);
                // println!("{next_pos:?}");
                match map.get(&next_pos) {
                    None => {
                        // field is empty at target so move there
                        current = next_pos;
                        found_possiblity = true;
                        break;
                    }
                    // TODO: check for start
                    Some(_) => {
                        // field is blocked by whatever, try next
                        continue;
                    }
                }
            }

            // if nothing possible. place sand forever.
            if !found_possiblity {
                map.insert(current, 'o');
                could_place = true;
                break;
            }
        }
        if !could_place {
            break;
        }
    }
    println!("{map:?}");
    let found_sand = map.iter().filter(|(_coords, &item)| item == 'o').count();
    found_sand as u64
}

fn solve_day2(input: &str) -> u64 {
    let mut map = parse(input);

    // add groundfloor
    let grondfloor_y = map.keys().map(|(_x, y)| y).max().unwrap() + 2;
    for x in -1_000_000..1_000_000 {
        map.insert((x, grondfloor_y), '#');
    }

    loop {
        // spawn sand
        // println!("spawn");
        let mut current = (500, 0);

        // let sand flow for some ticks
        loop {
            // check if able to move
            let possibilities = [(0, 1), (-1, 1), (1, 1)];
            let mut found_possiblity = false;
            for possibility in possibilities {
                let next_pos = (current.0 + possibility.0, current.1 + possibility.1);
                // println!("{next_pos:?}");
                match map.get(&next_pos) {
                    None => {
                        // field is empty at target so move there
                        current = next_pos;
                        found_possiblity = true;
                        break;
                    }
                    Some(_) => {
                        // field is blocked by whatever, try next
                        continue;
                    }
                }
            }

            // if nothing possible. place sand forever.
            if !found_possiblity && current == (500, 0) {
                map.insert(current, 'o');

                let found_sand = map.iter().filter(|(_coords, &item)| item == 'o').count();
                return found_sand as u64;
            } else if !found_possiblity {
                map.insert(current, 'o');
                break;
            }
        }
    }
}

fn parse(input: &str) -> HashMap<Coordinates, char> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let coords = line.split_terminator(" -> ");
        for (point, next_point) in coords.clone().zip(coords.skip(1)) {
            //println!("{point} {next_point}");
            let (x, y) = point.split_once(",").unwrap();
            let (next_x, next_y) = next_point.split_once(",").unwrap();

            let parse = |s: &str| s.parse::<i64>().unwrap();
            let order = |i1: i64, i2: i64| {
                if i1 > i2 {
                    (i2, i1)
                } else {
                    (i1, i2)
                }
            };

            let (x, next_x) = order(parse(x), parse(next_x));
            let (y, next_y) = order(parse(y), parse(next_y));

            for x in x..=next_x {
                for y in y..=next_y {
                    //println!("{x} {y}");
                    map.insert((x, y), '#');
                }
            }
        }
    }

    //println!("{map:?}");
    //println!("{}", map.len());
    map.insert((500, 0), '+');
    map
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day14.txt");
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
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 24);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let result = solve_day2(&input);
        assert_eq!(result, 93);
    }
}
