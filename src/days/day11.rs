use crate::{Solution, SolutionPair};
use gcd::Gcd;
use std::fmt::{self, Debug};

type Worry = u128;
type WorryOp = Box<dyn Fn(Worry) -> Worry>;

struct Monkey {
    items: Vec<Worry>,
    operation: WorryOp,
    divisor: Worry,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("operation_1", &(self.operation)(1))
            .field("operation_5", &(self.operation)(5))
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .field("divisor", &self.divisor)
            .finish()
    }
}

impl Monkey {
    fn new(
        items: Vec<Worry>,
        operation: WorryOp,
        divisor: Worry,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            operation,
            divisor,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        }
    }
}

fn parse_operation(op_input: &str) -> WorryOp {
    let op = &op_input[23..];
    if let Some(n) = op.strip_prefix("+ ") {
        if n == "old" {
            Box::new(move |x| x + x)
        } else {
            let n = n.parse::<Worry>().ok().unwrap();
            Box::new(move |x| x + n)
        }
    } else if let Some(n) = op.strip_prefix("* ") {
        if n == "old" {
            Box::new(move |x| x * x)
        } else {
            let n = n.parse::<Worry>().ok().unwrap();
            Box::new(move |x| x * n)
        }
    } else {
        unreachable!("'{op}'")
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let lines: Vec<_> = input.lines().collect();
    let monkies: Vec<_> = lines
        .chunks(7)
        .map(|chunk| {
            let starting_items: Vec<_> = chunk[1][18..]
                .split(",")
                .map(|s| s.trim().parse::<Worry>().expect(&format!("{s}")))
                .collect();

            let operation = parse_operation(chunk[2]);
            let divisor = chunk[3][20..].trim().parse::<Worry>().unwrap();
            let true_monkey = chunk[4][29..].trim().parse::<usize>().unwrap();
            let false_monkey = chunk[5][30..].trim().parse::<usize>().unwrap();
            Monkey::new(
                starting_items,
                operation,
                divisor,
                true_monkey,
                false_monkey,
            )
        })
        .collect();
    monkies
}

fn solve_day1(input: &str) -> u64 {
    let mut monkies = parse_input(input);
    for _round in 0..20 {
        for i in 0..monkies.len() {
            while monkies[i].items.len() > 0 {
                monkies[i].inspect_count += 1;
                let new_item_level = (monkies[i].operation)(monkies[i].items[0]) / 3;
                if new_item_level % monkies[i].divisor == 0 {
                    let idx = monkies[i].true_monkey;
                    monkies[idx].items.push(new_item_level);
                } else {
                    let idx = monkies[i].false_monkey;
                    monkies[idx].items.push(new_item_level);
                };
                monkies[i].items.remove(0);
            }
        }
        // println!("{_round}: ");
        // for (monkey_id, monkey) in monkies.iter().enumerate() {
        //     println!("{monkey_id} {:?}", monkey.borrow().items);
        // }
        // println!("");
    }

    let mut inspect_counts: Vec<_> = monkies
        .into_iter()
        .map(|monkey| monkey.inspect_count)
        .collect();
    inspect_counts.sort();
    // println!("{inspect_counts:?}");
    let top_2_inspect_counts_product = inspect_counts.into_iter().rev().take(2).product();
    top_2_inspect_counts_product
}

fn solve_day2(input: &str) -> u64 {
    let mut monkies = parse_input(input);
    let divisors = monkies.iter().map(|monkey| monkey.divisor);

    let lcm: u128 =
        divisors.clone().product::<u128>() / divisors.fold(0u128, |acc, divisor| acc.gcd(divisor));
    for _round in 0..10_000 {
        for i in 0..monkies.len() {
            while monkies[i].items.len() > 0 {
                monkies[i].inspect_count += 1;
                let new_item_level = (monkies[i].operation)(monkies[i].items[0]) % lcm;
                if new_item_level % monkies[i].divisor == 0 {
                    let idx = monkies[i].true_monkey;
                    monkies[idx].items.push(new_item_level);
                } else {
                    let idx = monkies[i].false_monkey;
                    monkies[idx].items.push(new_item_level);
                };
                monkies[i].items.remove(0);
            }
        }
        // println!("{_round}: ");
        // for (monkey_id, monkey) in monkies.iter().enumerate() {
        //     println!("{monkey_id} {:?}", monkey.borrow().items);
        // }
        // println!("");
    }

    let mut inspect_counts: Vec<_> = monkies
        .into_iter()
        .map(|monkey| monkey.inspect_count)
        .collect();
    inspect_counts.sort();
    // println!("{inspect_counts:?}");
    let top_2_inspect_counts_product = inspect_counts.into_iter().rev().take(2).product();
    top_2_inspect_counts_product
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day11.txt");
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
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3

      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0

      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3

      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1
        "};
        let result = solve_day1(&input);
        assert_eq!(result, 10605);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3

      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0

      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3

      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1
        "};
        let result = solve_day2(&input);
        assert_eq!(result, 2713310158);
    }
}
