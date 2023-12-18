use anyhow::{anyhow, Error, Ok, Result};
use std::{cmp, ops::RangeInclusive, str::FromStr};

// SPos stands for: "signed position", unlike Pos = (usize, usize)
type SPos = (isize, isize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> isize {
    let dig_steps: Vec<DigStep> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => {
                Some(trimmed.parse::<DigStep>().expect("failed to parse row"))
            }
            _ => None,
        })
        .collect();

    let mut curr: SPos = (0, 0);
    let mut trench_loop: Vec<SPos> = vec![curr];

    for DigStep { dir, num_steps, .. } in dig_steps {
        let next: SPos = match dir {
            Direction::Left => (curr.0, curr.1 - num_steps),
            Direction::Right => (curr.0, curr.1 + num_steps),
            Direction::Up => (curr.0 - num_steps, curr.1),
            Direction::Down => (curr.0 + num_steps, curr.1),
        };

        trench_loop.push(next);
        curr = next;
    }

    let (x_range, y_range) = bounding_ranges(&trench_loop);
    let mut count = 0;
    for x in x_range {
        for y in y_range.clone() {
            let pos = (x, y);
            if point_inside_contour(pos, &trench_loop) {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug)]
struct DigStep {
    dir: Direction,
    num_steps: isize,
}

impl FromStr for DigStep {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let dir: Direction = parts[0].parse()?;
        let num_steps: isize = parts[1].parse()?;

        Ok(Self { dir, num_steps })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(anyhow!("unknown direction")),
        }
    }
}

// ------------------------------------------------------------------------
// Taken from day 10, part 2
//
// Point in contour is improved with explicitly handling points on the edge
// ------------------------------------------------------------------------

// Implements the point-in-polygon algorithm by Dan Sunday based on winding numbers, see:
// https://en.wikipedia.org/wiki/Point_in_polygon
//
// If the winding number is:
// - = 0, point is outside the contour
// - != 0, point is either inside or on an edge
//
// Important: crucial part of the algorithm are edge crossing rules
// 1. an upward edge (end is above start) includes its start endpoint, and excludes its end point
// 2. a downward edge (end is below start) excludes its start endpoint, and includes its end point
// 3. horizontal edges are excluded
// 4. the edge-ray intersection point must be strictly right of the point of interest (`pos`)
fn point_inside_contour(pos: SPos, contour: &[SPos]) -> bool {
    if contour.len() < 3 {
        // Need at least 3 points to make a contour
        return false;
    }

    let contour_length = contour.len();
    let mut wn: isize = 0;
    for i in 0..(contour_length - 1) {
        let start = contour[i];
        let end = contour[i + 1];

        let left_probe = is_left(start, end, pos);
        let min_x = cmp::min(start.0, end.0);
        let max_x = cmp::max(start.0, end.0);
        let min_y = cmp::min(start.1, end.1);
        let max_y = cmp::max(start.1, end.1);
        if left_probe == 0 && pos.0 >= min_x && pos.0 <= max_x && pos.1 >= min_y && pos.1 <= max_y {
            return true;
        }

        if start.1 <= pos.1 {
            if end.1 > pos.1 && left_probe > 0 {
                // upward edge crossing of the point's ray: point must be to the left with respect
                // to the start -> end vector
                wn += 1;
            }
        } else if end.1 <= pos.1 && left_probe < 0 {
            // downward edge crossing of the point's ray: point must be to the right with respect
            // to the start -> end vector
            wn -= 1;
        }
    }

    wn != 0
}

// Check how the point lies with respect to the infinite line along the vector start -> end:
// - left side from the line, > 0
// - on the line, = 0
// - right side from the line, < 0
//
// Note: based on a cross product between two vectors, start -> end and start -> point
fn is_left(start: SPos, end: SPos, point: SPos) -> isize {
    (end.0 - start.0) * (point.1 - start.1) - (end.1 - start.1) * (point.0 - start.0)
}

// Find contour's bounding edges
fn bounding_ranges(contour: &[SPos]) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    contour.iter().for_each(|&(x, y)| {
        if min_x > x {
            min_x = x;
        }
        if min_y > y {
            min_y = y;
        }
        if max_x < x {
            max_x = x;
        }
        if max_y < y {
            max_y = y;
        }
    });

    (min_x..=max_x, min_y..=max_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "#;
        assert_eq!(62, process(input));
    }
}
