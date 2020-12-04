use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let mut pos = 0;
    let mut tree_ct = 0;
    if let Some(first) = input.lines().nth(0) {
        let width = first.chars().count();
        for l in input.lines() {
            if let Some(tob) = l.chars().nth(pos % width) {
                if tob == '#' { tree_ct += 1; }
                pos += 3;
            } else {
                unreachable!()
            }
        }
    }
    return tree_ct
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const INPUT: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
//     const CLAIM_1: Claim = Claim {
//         id: 1,
//         rect: Rectangle {
//             left: 1,
//             top: 3,
//             width: 4,
//             height: 4,
//         },
//     };
//     const CLAIM_2: Claim = Claim {
//         id: 2,
//         rect: Rectangle {
//             left: 3,
//             top: 1,
//             width: 4,
//             height: 4,
//         },
//     };
//     const CLAIM_3: Claim = Claim {
//         id: 3,
//         rect: Rectangle {
//             left: 5,
//             top: 5,
//             width: 2,
//             height: 2,
//         },
//     };

//     #[test]
//     fn parse_example() {
//         assert_eq!(parse(INPUT).unwrap(), vec![CLAIM_1, CLAIM_2, CLAIM_3,]);
//     }

//     #[test]
//     fn overlaps() {
//         assert_eq!(
//             CLAIM_1.rect.overlaps(&CLAIM_2.rect),
//             Some(Rectangle {
//                 top: 3,
//                 left: 3,
//                 width: 2,
//                 height: 2,
//             })
//         );
//         assert_eq!(CLAIM_1.rect.overlaps(&CLAIM_3.rect), None);
//         assert_eq!(CLAIM_2.rect.overlaps(&CLAIM_3.rect), None);
//     }

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(&[CLAIM_1, CLAIM_2, CLAIM_3,]), 4);
//     }

//     #[test]
//     fn part3_example() {
//         assert_eq!(part2(&[CLAIM_1, CLAIM_2, CLAIM_3,]).unwrap(), 3);
//     }
// }
