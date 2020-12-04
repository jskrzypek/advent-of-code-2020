use aoc_runner_derive::{aoc};
use itertools::Itertools;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            if let Some((range, req, pw)) = l.split(" ").collect_tuple() {
                if let Some(ch) = req.chars().nth(0) {
                    if let Some((Ok(min), Ok(max))) = range.split('-').map(|l| l.parse::<usize>()).collect_tuple() {
                        return (min..=max).contains(&pw.matches(ch).count())
                    }
                }
            }
            false
        })
        .filter(|x| *x)
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            if let Some((range, req, pw)) = l.split(" ").collect_tuple() {
                if let Some(ch) = req.chars().nth(0) {
                    if let Some((Ok(p1), Ok(p2))) = range.split('-').map(|l| l.parse::<usize>()).collect_tuple() {
                        if let Some(c1) = pw.chars().nth(p1-1) {
                            if let Some(c2) = pw.chars().nth(p2-1) {
                                return (ch == c1 || ch == c2) && c1 != c2
                            }
                            return ch == c1
                        }
                    }
                }
            }
            false
        })
        .filter(|x| *x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1);
    }
}
