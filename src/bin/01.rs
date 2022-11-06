use std::io::BufRead;

use itertools::Itertools;

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let nums = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (part1(&nums), part2(&nums))
}

fn part1(nums: &[i32]) -> usize {
    nums.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

fn part2(nums: &[i32]) -> usize {
    nums.iter()
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count()
}
