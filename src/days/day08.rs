use crate::{Solution, SolutionPair};
type ParsedMap = Vec<Vec<(bool, u64)>>;

fn solve_day1(input: &ParsedMap) -> u64 {
    let visibile_trees = input.iter().flatten().filter(|&&(e, _)| e).count();
    visibile_trees as u64
}
fn solve_day2(input: &ParsedMap) -> u64 {
    let max_scenic_score = input
        .iter()
        .flatten()
        .map(|&(_, scenic_score)| scenic_score)
        .max()
        .unwrap_or(0);
    max_scenic_score
}

fn scenic_calc<'a, I>(iter: I, tree_size: &u32) -> usize
where
    I: Iterator<Item = &'a u32>,
{
    let mut scenic = 0;
    for (i, other_tree_size) in iter.enumerate() {
        scenic = i + 1;
        if other_tree_size >= tree_size {
            break;
        }
    }
    scenic
}

fn is_visible_calc<'a, I>(iterable: I, tree_size: &u32) -> bool
where
    I: IntoIterator<Item = &'a u32>,
{
    iterable
        .into_iter()
        .all(|other_tree_size: &u32| other_tree_size < tree_size)
}

fn visibility_map(input: &Vec<Vec<u32>>) -> ParsedMap {
    input
        .iter()
        .enumerate()
        .map(|(row, rowdata)| {
            rowdata
                .iter()
                .enumerate()
                .map(|(column, tree_size)| {
                    let up = (0..row).rev().map(|row| &input[row][column]);
                    let up_visible = is_visible_calc(up.clone(), tree_size);
                    let up_scenic = scenic_calc(up, tree_size);

                    let down = ((row + 1)..input.len()).map(|row| &input[row][column]);
                    let down_visible = is_visible_calc(down.clone(), tree_size);
                    let down_scenic = scenic_calc(down, tree_size);

                    let left = ((0..column).rev()).map(|column| &input[row][column]);
                    let left_visible = is_visible_calc(left.clone(), tree_size);
                    let left_scenic = scenic_calc(left, tree_size);

                    let right = ((column + 1)..rowdata.len()).map(|column| &input[row][column]);
                    let right_visible = is_visible_calc(right.clone(), tree_size);
                    let right_scenic = scenic_calc(right, tree_size);

                    let visible = up_visible || down_visible || left_visible || right_visible;
                    let scenic = up_scenic * down_scenic * left_scenic * right_scenic;

                    (visible, scenic as u64)
                })
                .collect()
        })
        .collect()
}

fn parse(input: &str) -> ParsedMap {
    let rows = input.lines();
    let result: Vec<Vec<u32>> = rows
        .into_iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    visibility_map(&result)
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day08.txt");
    let parsed_input = parse(input);
    let sol1 = solve_day1(&parsed_input);
    let sol2 = solve_day2(&parsed_input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
            30373
            25512
            65332
            33549
            35390
        "};
        let parsed_input = parse(input);
        let result = solve_day1(&parsed_input);
        assert_eq!(result, 21);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            30373
            25512
            65332
            33549
            35390
        "};
        let parsed_input = parse(input);
        let result = solve_day2(&parsed_input);
        assert_eq!(result, 8);
    }
}
