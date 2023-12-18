use anyhow::{anyhow, bail, Error, Ok, Result};
use std::{cmp, collections::HashSet, str::FromStr};

// SPos stands for: "signed position", unlike Pos = (usize, usize)
type SPos = (isize, isize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
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

    // The trench_loop is too big to iterate naively through all the points in the bounding
    // rectange, so we need a better strategy. We can split the entire canvas into a sequence of
    // non-overlapping ranges over both directions, and make decision on counting points based on
    // the entire ranges instead of individual points. It is possible to do that because the
    // polygon is made of segments that are 0 or 90 degrees with respect to its neighbors, and
    // there will be no edges at irregular angles.
    //
    // Note: here 'row' index is considered to be x, while 'col' index is considered to be y
    // (rotated coordinate system).
    let (xs, ys) = get_sorted_unique_coords(&trench_loop);
    let max_x = *xs.last().expect("the x range is empty");
    let max_y = *ys.last().expect("the y range is empty");

    // iterate over all non-overlapping ranges in both directions
    let mut count = 0;
    for x_range in xs.windows(2) {
        let x_start = x_range[0];

        // avoid double-counting of the last point, except when we are looking at the last range
        let x_end = if x_range[1] < max_x {
            x_range[1] - 1
        } else {
            x_range[1]
        };

        for y_range in ys.windows(2) {
            let y_start = y_range[0];

            // avoid double-counting of the last point, except when we are looking at the last range
            let y_end = if y_range[1] < max_y {
                y_range[1] - 1
            } else {
                y_range[1]
            };

            // check if top-left point is inside the contour
            if point_inside_contour((x_start, y_start), &trench_loop) {
                // if the left vertical edge is inside contour, we count all points in the x_range,
                // otherwise only the top-left point
                let count_x = if point_inside_contour((x_end, y_start), &trench_loop) {
                    x_end - x_start + 1
                } else {
                    1
                };

                // if the top horizontal edge is inside contour, we count all points in the
                // y_range, otherwise only the top-left point
                let count_y = if point_inside_contour((x_start, y_end), &trench_loop) {
                    y_end - y_start + 1
                } else {
                    1
                };

                // finally, counting depends on the bottom-right point
                if point_inside_contour((x_end, y_end), &trench_loop) {
                    // here, both x_range and y_range are inside the contour, so the number of
                    // points is the product of the corresponding counts (even though the top-left
                    // point is shared between the ranges, there is no double-counting when
                    // calculate the area defined by the ranges)
                    count += count_x * count_y;
                } else {
                    // otherwise, only the edges should be included, but not the interior (we have
                    // to subtract 1 here to avoid double-counting of the top-left point that is
                    // shared between both ranges)
                    count += count_x + count_y - 1;
                }
            }
        }
    }
    count
}

fn get_sorted_unique_coords(contour: &[SPos]) -> (Vec<isize>, Vec<isize>) {
    let mut unique_x: HashSet<isize> = HashSet::new();
    let mut unique_y: HashSet<isize> = HashSet::new();

    for (x, y) in contour {
        unique_x.insert(*x);
        unique_y.insert(*y);
    }

    let mut xs: Vec<isize> = Vec::from_iter(unique_x);
    let mut ys: Vec<isize> = Vec::from_iter(unique_y);
    xs.sort();
    ys.sort();

    (xs, ys)
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

        let trim_chars: &[_] = &['(', ')', '#'];
        let trimmed = parts.last().unwrap().trim_matches(trim_chars);
        let dir_str = match trimmed.chars().nth(5).unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => bail!("could not parse direction"),
        };

        let dir: Direction = dir_str.parse()?;
        let num_steps: isize = isize::from_str_radix(&trimmed[..5], 16)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
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
        assert_eq!(952408144115, process(input));
    }
}
