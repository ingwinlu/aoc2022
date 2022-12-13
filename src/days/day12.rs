use std::collections::{HashSet, VecDeque};

use crate::{Solution, SolutionPair};

type Coordinates = (i32, i32);

const LOWEST_ELEVATION: u32 = 'a' as u32;
const HIGHEST_ELEVATION: u32 = 'z' as u32;

struct Map {
    elevations: Vec<Vec<u32>>,
    current_position: Coordinates,
    best_signal_location: Coordinates,
    width: usize,
    height: usize,
}

impl Map {
    fn neighbor_elevations(&self, current_position: Coordinates) -> Vec<(Coordinates, u32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| (current_position.0 + x, current_position.1 + y))
            .filter(|(x, y)| {
                *x >= 0 && (*x as usize) < self.width && *y >= 0 && (*y as usize) < self.height
            })
            .map(|(x, y)| ((x, y), self.elevations[y as usize][x as usize]))
            .collect()
    }
}

fn parse(input: &str) -> Map {
    let mut current_position = (0, 0);
    let mut best_signal_location = (0, 0);
    let mut elevations = Vec::new();

    for (y, row) in input.lines().enumerate() {
        let mut row_elevations = Vec::with_capacity(row.len());

        for (x, elevation) in row.chars().enumerate() {
            match elevation {
                'S' => {
                    current_position = (x as i32, y as i32);
                    row_elevations.push(LOWEST_ELEVATION)
                }
                'E' => {
                    best_signal_location = (x as i32, y as i32);
                    row_elevations.push(HIGHEST_ELEVATION)
                }
                elevation => row_elevations.push(elevation as u32),
            };
        }

        elevations.push(row_elevations);
    }

    let width = elevations[0].len();
    let height = elevations.len();

    Map {
        elevations,
        current_position,
        best_signal_location,
        width,
        height,
    }
}

fn solve_day1(input: &str) -> u64 {
    let map = parse(input);
    let current_position = map.current_position;
    let mut queue = VecDeque::from(vec![(current_position, LOWEST_ELEVATION, 0)]);
    let mut visited: HashSet<Coordinates> = HashSet::from([current_position]);

    while !queue.is_empty() {
        let (current_position, current_elevation, path_length) = queue.pop_front().unwrap();

        if current_position == map.best_signal_location {
            return path_length;
        }

        for (neighbor_position, neighbor_elevation) in map.neighbor_elevations(current_position) {
            if neighbor_elevation as i32 - current_elevation as i32 <= 1
                && !visited.contains(&neighbor_position)
            {
                queue.push_back((neighbor_position, neighbor_elevation, path_length + 1));
                visited.insert(neighbor_position);
            }
        }
    }

    0
}
fn solve_day2(input: &str) -> u64 {
    let map = parse(input);
    let current_position = map.best_signal_location;
    let mut queue = VecDeque::from(vec![(current_position, HIGHEST_ELEVATION, 0)]);
    let mut visited: HashSet<Coordinates> = HashSet::from([current_position]);

    while !queue.is_empty() {
        let (current_position, current_elevation, path_length) = queue.pop_front().unwrap();

        if current_elevation == LOWEST_ELEVATION {
            return path_length;
        }

        for (neighbor_position, neighbor_elevation) in map.neighbor_elevations(current_position) {
            if current_elevation as i32 - neighbor_elevation as i32 <= 1
                && !visited.contains(&neighbor_position)
            {
                queue.push_back((neighbor_position, neighbor_elevation, path_length + 1));
                visited.insert(neighbor_position);
            }
        }
    }

    0
}
pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day12.txt");
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
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 31);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "};
        let result = solve_day2(&input);
        assert_eq!(result, 29);
    }
}
