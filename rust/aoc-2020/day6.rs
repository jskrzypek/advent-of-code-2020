use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    input
    .split("\n\n")
    .map(|s| -> BTreeSet<char> {s.replace('\n', "").chars().collect()})
    .map(|g| g.len())
    .fold(0, |acc, n| acc + n)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    input
    .split("\n\n")
    .map(|s| {
        s
        .lines()
        .map(|l| {
            ('a'..='z')
                .map(|c| match l.contains(c) {
                    true => "1",
                    false => "0"
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .map(|b| u32::from_str_radix(&b, 2).unwrap())
        .fold(u32::MAX, |acc, b| acc & b)
        .count_ones()
    })
    .fold(0, |acc, n| acc + n)
}