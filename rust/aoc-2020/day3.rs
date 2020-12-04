use aoc_runner_derive::aoc;

fn check_slope(right: usize, down: usize, input: &str) -> usize {
    let mut pos = 0;
    let mut tree_ct = 0;
    if let Some(first) = input.lines().nth(0) {
        let width = first.chars().count();
        let mut lines = input.lines();
        loop {
            if let Some(l) = lines.next() {
                if let Some(tob) = l.chars().nth(pos % width) {
                    if tob == '#' { tree_ct += 1; }
                    pos += right;
                } else {
                    unreachable!()
                }
                for _ in 1..down {
                    lines.next();
                }
            } else {
                break;
            }
        }
    }
    return tree_ct
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    return check_slope(3, 1, input)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let mut prod = 1;
    for (right, down) in slopes.iter() {
        let tree_ct = check_slope(*right, *down, input);
        println!("{}, {}: {}", *right, *down, tree_ct);
        prod *= tree_ct;
    }
    return prod
}