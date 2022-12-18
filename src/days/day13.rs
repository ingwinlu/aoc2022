use std::cmp::Ordering;

use crate::{Solution, SolutionPair};

#[derive(Debug, Clone, Eq)]
pub enum Packet {
    Int(u32),
    List(Vec<Self>),
}

impl Packet {
    fn parse_one(s: &str) -> (Self, &str) {
        if let Some(mut s) = s.strip_prefix('[') {
            let mut list = vec![];
            if let Some(trailing) = s.strip_prefix(']') {
                return (Self::List(list), trailing);
            }

            loop {
                let (value, trailing) = Self::parse_one(s);
                list.push(value);
                let (c, trailing) = {
                    let mut chars = trailing.chars();
                    (chars.next(), chars.as_str())
                };
                match c {
                    Some(',') => (),
                    Some(']') => return (Self::List(list), trailing),
                    _ => unreachable!(),
                }
                s = trailing;
            }
        } else {
            let terminator = s.find([',', ']']).unwrap_or(s.len());
            let (s, trailing) = s.split_at(terminator);
            (Self::Int(s.parse().expect(s)), trailing)
        }
    }

    pub fn as_slice(&self) -> &[Self] {
        if let Self::List(list) = self {
            list.as_slice()
        } else {
            std::slice::from_ref(self)
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Int(a), Self::Int(b)) = (self, other) {
            a.cmp(b)
        } else {
            self.as_slice().cmp(other.as_slice())
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn solve_day1(input: &str) -> u64 {
    let packages: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse_one(line).0)
        .collect();

    let sorted: u64 = packages
        .chunks(2)
        .map(|a| a[0] <= a[1])
        .enumerate()
        .filter_map(|(idx_0, ordered)| {
            if ordered {
                Some(idx_0 as u64 + 1)
            } else {
                None
            }
        })
        .sum();

    sorted
}

fn solve_day2(input: &str) -> u64 {
    let mut packages: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse_one(line).0)
        .collect();
    let devider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let devider_6 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packages.push(devider_2.clone());
    packages.push(devider_6.clone());

    packages.sort();

    let mut decoder_key = 1;
    packages.iter().enumerate().for_each(|(idx, package)| {
        if package == &devider_2 || package == &devider_6 {
            decoder_key *= idx + 1;
        }
    });

    decoder_key as u64
}
pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day13.txt");
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
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "};
        let result = solve_day2(&input);
        assert_eq!(result, 140);
    }
}
