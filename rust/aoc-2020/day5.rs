use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug)]
struct Seat {
    row: u16,
    col: u16,
    sid: u16,
    cod: String,
}

#[aoc_generator(day5)]
// attempt 1... doesn't need to be this hard...
// fn parse_input_day1(input: &str) -> Result<Vec<Seat>, &'static str> {
//     input.lines().map(|l| -> Result<Seat, &'static str> {
//         let (r, c) = l.split_at(7);
//         println!("{:?} {:?}",r,c);
//         let br = r.replace('F', "0").replace('B', "1");
//         if let Ok(row) = u16::from_str_radix(&br, 2) {
//             println!("{:?} {:?}",r,br);
//             let bc = c.replace('L', "0").replace('R', "1");
//             if let Ok(col) = u16::from_str_radix(&bc, 2) {
//                 if let Ok(col) = u16::from_str_radix(&bc, 2) {
//                     println!("{:?} {:?}",c,bc);
//                     return Ok(Seat { row, col, sid: row * 8u16 + col })
//                 }
//             } else {
//                 return Err("failed to parse col")
//             }
//         } else {
//             return Err("failed to parse row")
//         }
//     }).collect()
// }
fn parse_input_day1(input: &str) -> Result<Vec<Seat>, ParseIntError> {
    input
        .replace('F', "0").replace('B', "1")
        .replace('L', "0").replace('R', "1")
        .lines()
        .map(|cod| {
            match u16::from_str_radix(cod, 2) {
                Ok(sid) => Ok(Seat {cod: (*cod).to_string(), sid, row: sid / 8u16, col: sid % 8u16 }),
                Err(e) => Err(e),
            }
        })
        .collect()

}

#[aoc(day5, part1)]
fn part1(input: &[Seat]) -> u16 {
    input.iter().fold(0u16, |acc, s| acc.max(s.sid))
}

#[aoc(day5, part2)]
fn part2(input: &[Seat]) -> u16 {
    let sections = input.iter().fold(Vec::<Range<u16>>::new(), |mut acc, Seat {sid, ..}| {
        if acc.is_empty() {
            acc.push(*sid..(*sid+1));
        } else {
            let idx = acc.as_slice().binary_search_by(|a| {
                match *a {
                    Range { start, end } if (start..end).contains(sid) => Ordering::Equal,
                    Range { end, .. } if end == *sid => Ordering::Equal,
                    Range { start, .. } if start == *sid + 1 => Ordering::Equal,
                    Range { start, .. } if start > *sid => Ordering::Greater,
                    Range { end, .. } if end < *sid => Ordering::Less,
                    Range { .. } => unreachable!(),
                }
            });
            match idx {
                Err(i) => acc.insert(i, *sid..(*sid+1)),
                Ok(j) => {
                    let mut idx_range = j..j+1;
                    let mut arange: Range<u16> = acc[j].start..acc[j].end;
                    if !acc[j].contains(sid) {
                        if acc[j].end == *sid {
                            arange = acc[j].start..*sid + 1;
                        } else if acc[j].start == *sid + 1 {
                            arange = *sid..acc[j].end;
                        }
                    } 
                    if let Some(ai) = acc.get(j-1) {
                        if ai.end == arange.start {
                            idx_range.start = j-1;
                            arange = ai.start..arange.end;
                        }
                    }
                    if let Some(ak) = acc.get(j+1) {
                        if arange.end == ak.start {
                            idx_range.end = j+2;
                            arange = arange.start..ak.end;
                        }
                    }
                    acc.splice(idx_range, [arange].iter().cloned());
                },
            }

        }
        acc
    });
    println!("sections {:?}", sections);
    sections.get(0).unwrap_or(&(0..1)).end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1(b"dabAcCaCBAcCcaDA"), 10);
        assert_eq!(part1_stack(b"dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(b"dabAcCaCBAcCcaDA"), Some(4));
        assert_eq!(part2_stack(b"dabAcCaCBAcCcaDA"), Some(4));
    }
}
