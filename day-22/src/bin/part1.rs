use anyhow::{anyhow, Error, Ok, Result};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Pos = (usize, usize, usize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let bricks = parse_input(input);
    let settled = settle_down(bricks);
    let overlaps = find_overlaps(&settled);

    let mut seen: HashSet<Brick> = HashSet::new();
    for (brick, (above, _)) in overlaps.iter() {
        if above.is_empty() {
            seen.insert(*brick);
            continue;
        }

        if above.iter().all(|brick_above| {
            let (_, above_below) = overlaps.get(brick_above).unwrap();
            above_below.iter().any(|b| b != brick)
        }) {
            seen.insert(*brick);
        }
    }

    seen.len()
}

fn find_overlaps(bricks: &[Brick]) -> HashMap<Brick, (Vec<Brick>, Vec<Brick>)> {
    let mut overlaps = HashMap::new();

    for brick in bricks {
        let mut below: Vec<Brick> = Vec::new();
        let mut above: Vec<Brick> = Vec::new();
        for other in bricks {
            if brick.projection_overlap(other) {
                if brick.start.2 == (other.end.2 + 1) {
                    below.push(*other);
                }

                if other.start.2 == (brick.end.2 + 1) {
                    above.push(*other);
                }
            }
        }
        overlaps.insert(*brick, (above, below));
    }

    overlaps
}

fn settle_down(mut bricks: Vec<Brick>) -> Vec<Brick> {
    let mut settled: Vec<Brick> = Vec::new();
    bricks.sort();

    for mut brick in bricks {
        let brick_dz = brick.end.2 - brick.start.2;
        if let Some(under) = settled
            .iter()
            .filter(|b| b.projection_overlap(&brick))
            .max()
        {
            brick.start.2 = under.end.2 + 1;
            brick.end.2 = brick.start.2 + brick_dz;
        } else {
            brick.start.2 = 1;
            brick.end.2 = brick.start.2 + brick_dz;
        }
        settled.push(brick);
    }

    settled
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => {
                Some(trimmed.parse().unwrap_or_else(|err| panic!("{err}")))
            }
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    start: Pos,
    end: Pos,
}

impl Brick {
    fn projection_overlap(&self, other: &Brick) -> bool {
        let x_overlap = self.start.0 <= other.end.0 && self.end.0 >= other.start.0;
        let y_overlap = self.start.1 <= other.end.1 && self.end.1 >= other.start.1;
        x_overlap && y_overlap
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // order by z-coordinate
        self.end.2.cmp(&other.end.2)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start_str, end_str)) = s.split_once('~') {
            let start = parse_pos(start_str)?;
            let end = parse_pos(end_str)?;

            return Ok(Self { start, end });
        }
        Err(anyhow!("Failed to parse brick: {s}"))
    }
}

fn parse_pos(pos_str: &str) -> Result<Pos> {
    let nums: Vec<&str> = pos_str.split(',').collect();
    let x = nums[0].parse()?;
    let y = nums[1].parse()?;
    let z = nums[2].parse()?;
    Ok((x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "#;
        assert_eq!(5, process(input));
    }
}
