use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::ops::Range;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn find_sum<'a>(target: i32, sorted: &'a [&i32]) -> Option<(&'a i32, &'a i32)> {
    let Range { mut start, mut end } = sorted.as_ptr_range();
    loop {
        if end <= start { break };
        unsafe {
        match *start + *end.sub(1) {
            s if s == target => return Some((*start, *end.sub(1))),
            s if s < target => start = start.add(1),
            s if s > target => end = end.sub(1),
            _ => break
        }
        }
    }
    None
}

#[aoc(day1, part1)]
fn part1(entries: &[i32]) -> i32 {
    let sorted = entries.into_iter().sorted().collect_vec();
    if let Some((start, end)) = find_sum(2020, sorted.as_slice()) {
        return start * end
    }
    -1
}

#[aoc(day1, part2)]
fn part2(entries: &[i32]) -> i32 {
    let sorted = entries.into_iter().sorted().collect_vec();
    let mut slice = sorted.as_slice();
    if let Some(head) = slice.first() {
        while let Some((right, prev)) = slice.split_last() {
            if *right < &(2020 - (2 * *head)) {
                if let Some((start, end)) = find_sum(2020 - *right, slice) {
                    println!("{} {} {}", start, end, right);
                    return start * end * *right
                }
            }
            slice = prev;
            continue;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[299, 366, 675, 979, 1456, 1721]), 514579);
        assert_eq!(part1(&[1010,1010]), 1020100);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part1(&[299, 366, 675, 979, 1456, 1721]), 241861950);
    }

    // #[test]
    // fn part2_fnv_example() {
    //     assert_eq!(part2_fnv(&[1, -2, 3, 1]), 2);
    //     assert_eq!(part2_fnv(&[1, -1]), 0);
    //     assert_eq!(part2_fnv(&[3, 3, 4, -2, -4]), 10);
    //     assert_eq!(part2_fnv(&[-6, 3, 8, 5, -6]), 5);
    //     assert_eq!(part2_fnv(&[7, 7, -2, -7, -4]), 14);
    // }
}
