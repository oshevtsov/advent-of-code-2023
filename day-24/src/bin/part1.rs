use core::f64;
use std::str::FromStr;

use anyhow::{anyhow, Error};
fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input, 200000000000000, 400000000000000);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str, min_c: isize, max_c: isize) -> usize {
    let hailstones = parse_input(input);
    count_trajectory_intersects(&hailstones, min_c, max_c)
}

fn parse_input(input: &str) -> Vec<Hailstone> {
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

fn count_trajectory_intersects(hailstones: &[Hailstone], min_c: isize, max_c: isize) -> usize {
    let mut count = 0;

    // take all pairs of hailstones
    hailstones.iter().enumerate().for_each(|(i, h1)| {
        hailstones.iter().skip(i + 1).for_each(|h2| {
            if h1.intersects(h2, min_c, max_c) {
                count += 1;
            }
        });
    });

    count
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone, min_c: isize, max_c: isize) -> bool {
        if self.is_parallel(other) {
            return false;
        }

        // find intersection point and check that it is in bounds
        let (slope, offset) = self.xy_trajectory();
        let (other_slope, other_offset) = other.xy_trajectory();

        let x_i = (other_offset - offset) / (slope - other_slope);
        let y_i = slope * x_i + offset;
        let min_bound = min_c as f64;
        let max_bound = max_c as f64;
        let num_steps = (x_i - self.pos[0] as f64) / self.vel[0] as f64;
        let num_steps_other = (x_i - other.pos[0] as f64) / other.vel[0] as f64;

        if num_steps < 0.0 || num_steps_other < 0.0 {
            return false;
        }

        (x_i >= min_bound && x_i <= max_bound) && (y_i >= min_bound && y_i <= max_bound)
    }

    fn is_parallel(&self, other: &Hailstone) -> bool {
        let cross = self.vel[0] * other.vel[1] - self.vel[1] * other.vel[0];
        cross == 0
    }

    fn xy_trajectory(&self) -> (f64, f64) {
        let slope: f64 = self.vel[1] as f64 / self.vel[0] as f64;
        let offset = self.pos[1] as f64 - slope * (self.pos[0] as f64);
        (slope, offset)
    }
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((pos_str, vel_str)) = s.split_once(" @ ") {
            let mut pos: [isize; 3] = [0; 3];
            for (idx, pos_c) in pos_str.split(',').enumerate() {
                pos[idx] = pos_c.trim().parse::<isize>()?;
            }

            let mut vel: [isize; 3] = [0; 3];
            for (idx, vel_c) in vel_str.split(',').enumerate() {
                vel[idx] = vel_c.trim().parse::<isize>()?;
            }
            return Ok(Self { pos, vel });
        }
        Err(anyhow!("Failed to parse hailstone: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_intersects_inside() {
        let input = r#"
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        "#;
        let hailstones = parse_input(input);

        assert!(hailstones[0].intersects(&hailstones[1], 7, 27));
        assert!(hailstones[0].intersects(&hailstones[2], 7, 27));
        assert!(!hailstones[0].intersects(&hailstones[3], 7, 27));
        assert!(!hailstones[0].intersects(&hailstones[4], 7, 27));
        assert!(!hailstones[1].intersects(&hailstones[2], 7, 27));
        assert!(!hailstones[1].intersects(&hailstones[3], 7, 27));
        assert!(!hailstones[1].intersects(&hailstones[4], 7, 27));
        assert!(!hailstones[2].intersects(&hailstones[3], 7, 27));
        assert!(!hailstones[2].intersects(&hailstones[4], 7, 27));
        assert!(!hailstones[3].intersects(&hailstones[4], 7, 27));
    }

    #[test]
    fn part1_process() {
        let input = r#"
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        "#;
        assert_eq!(2, process(input, 7, 27));
    }
}
