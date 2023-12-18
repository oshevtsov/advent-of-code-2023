// ------------------------------------------------------------------
// Alternative solution based on:
// - Pick's theorem: https://en.wikipedia.org/wiki/Pick%27s_theorem
// - Shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
// ------------------------------------------------------------------

use anyhow::{anyhow, Error, Ok, Result};
use std::str::FromStr;

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

    let area = calculate_contour_area(&trench_loop);
    let perimeter = calculate_contour_perimeter(&trench_loop);
    let num_interior_points = area - perimeter / 2 + 1;

    perimeter + num_interior_points
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

// ----------------------------------------------------------------------------------------------
// The result mentioned in the two links at the top allow for an alternative solution:
// - Pick's theorem: for a simple polygon, area is related its perimeter and number of interior
// points (points that are inside the polygon, but not on its edge) as `A = I + P/2 -1`,
// where A - area, P - perimeter, I - number of interior points.
//
// - Shoelace formula: allows calculating a simple polygon's area A, using it vertices by summing
// up oriented areas of rectangles comprised of coordinate system origin O and the two adjacent
// vertices.
// ----------------------------------------------------------------------------------------------

// It assumes that the last element in the contour is the same as the first.
fn calculate_contour_perimeter(contour: &[SPos]) -> isize {
    let contour_len = contour.len();
    let mut perimeter = 0;
    for i in 0..(contour_len - 1) {
        let (x1, y1) = contour[i];
        let (x2, y2) = contour[i + 1];

        perimeter += (x2 - x1).abs() + (y2 - y1).abs();
    }
    perimeter
}

// Calculate area using Shoelace formula, see https://en.wikipedia.org/wiki/Shoelace_formula.
// It assumes that the last element in the contour is the same as the first. The original formula
// requires that the contour is walked counter-clockwise when iterating over vertices (otherwise,
// the answer is negative). We take absolute value to make direction irrelevant.
fn calculate_contour_area(contour: &[(isize, isize)]) -> isize {
    let contour_len = contour.len();

    let mut double_area: isize = 0;
    for i in 0..(contour_len - 1) {
        double_area += contour[i].0 * contour[i + 1].1 - contour[i + 1].0 * contour[i].1;
    }

    double_area.abs() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_alt_process() {
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
