use std::{collections::HashSet, ops::RangeInclusive, usize};

type Pos = (usize, usize);
type SPos = (isize, isize);
type PosInc = (isize, isize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> usize {
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .collect();

    let pipe_map = PipeMap::from_input(lines);
    pipe_map.count_points_in_contour()
}

fn increment(start: Pos, inc: PosInc) -> Pos {
    let (r, c) = start;
    let (r_inc, c_inc) = inc;
    let n_r: usize = if r_inc < 0 {
        r - r_inc.unsigned_abs()
    } else {
        r + r_inc.unsigned_abs()
    };

    let n_c: usize = if c_inc < 0 {
        c - c_inc.unsigned_abs()
    } else {
        c + c_inc.unsigned_abs()
    };
    (n_r, n_c)
}

fn process_origin(map: &mut [Vec<Symbol>]) -> (Pos, (Pos, Pos)) {
    // Find origin location
    let mut origin_row: usize = 0;
    let mut origin_col: usize = 0;
    for (row_idx, row) in map.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&s| s == Symbol::Start) {
            origin_row = row_idx;
            origin_col = col_idx;
            break;
        }
    }
    let origin = (origin_row, origin_col);

    // Find all neighboring tiles that are pipes (ignore ground tiles)
    let neighbor_pipes_pos: Vec<(PipeType, Pos)> = [
        (origin_row - 1, origin_col),
        (origin_row + 1, origin_col),
        (origin_row, origin_col - 1),
        (origin_row, origin_col + 1),
    ]
    .into_iter()
    .filter_map(|pos| match map[pos.0][pos.1] {
        Symbol::Start => panic!("Can't have another start tile in the same map"),
        Symbol::Ground => None,
        Symbol::Pipe(p) => Some((p, pos)),
    })
    .collect();

    // Identify the origin pipe type and positions of its two neighbors that match
    for (idx, pipe_pos_1) in neighbor_pipes_pos.iter().enumerate() {
        for pipe_pos_2 in neighbor_pipes_pos
            .iter()
            .take(neighbor_pipes_pos.len())
            .skip(idx + 1)
        {
            if let Some(pipetype) = find_common_next(origin, *pipe_pos_1, *pipe_pos_2) {
                map[origin_row][origin_col] = Symbol::Pipe(pipetype);
                return (origin, (pipe_pos_1.1, pipe_pos_2.1));
            }
        }
    }
    panic!("Failed to process origin");
}

// Based on two (pipe, position) pairs, find the pipe type of their neighbor at common_pos that
// matches both pipes
fn find_common_next(
    common_pos: Pos,
    pipe_pos_1: (PipeType, Pos),
    pipe_pos_2: (PipeType, Pos),
) -> Option<PipeType> {
    let candidates_1: Vec<PipeType> = find_next_at(pipe_pos_1, common_pos);
    let candidates_2: Vec<PipeType> = find_next_at(pipe_pos_2, common_pos);

    let mut maybe_pipetype: Option<PipeType> = None;
    for candidate_1 in candidates_1.iter() {
        for candidate_2 in candidates_2.iter() {
            if candidate_1 == candidate_2 {
                maybe_pipetype = Some(*candidate_1);
            }
        }
    }

    maybe_pipetype
}

// Find all matching neighbors that are at a given position
fn find_next_at(pipe_pos: (PipeType, Pos), next_pos: Pos) -> Vec<PipeType> {
    let (pipe, pos) = pipe_pos;
    pipe.next_allowed()
        .iter()
        .filter_map(|(p, inc)| {
            if increment(pos, *inc) == next_pos {
                Some(*p)
            } else {
                None
            }
        })
        .collect()
}

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
fn point_inside_contour(pos: (isize, isize), contour: &[(isize, isize)]) -> bool {
    if contour.len() < 3 {
        // Need at least 3 points to make a contour
        return false;
    }

    let contour_length = contour.len();
    let mut wn: isize = 0;
    for i in 0..contour_length {
        let start = contour[i];
        let end = contour[(i + 1) % contour_length];

        if start.1 <= pos.1 {
            if end.1 > pos.1 && is_left(start, end, pos) > 0 {
                // upward edge crossing of the point's ray: point must be to the left with respect
                // to the start -> end vector
                wn += 1;
            }
        } else if end.1 <= pos.1 && is_left(start, end, pos) < 0 {
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

#[derive(Debug, Clone)]
struct PipeMap {
    map: Vec<Vec<Symbol>>,
    origin: (usize, usize),
    // neighbors that matche origin
    origin_next_1: (usize, usize),
    origin_next_2: (usize, usize),
}

impl PipeMap {
    fn from_input(lines: Vec<&str>) -> Self {
        // extend the map with ground tiles on margins for safe walking
        let row_length = lines[0].len() + 2;
        let num_rows = lines.len() + 2;
        let mut map: Vec<Vec<Symbol>> = Vec::with_capacity(num_rows);
        map.push(vec![Symbol::Ground; row_length]);
        map.extend(lines.iter().map(|l| {
            let mut row: Vec<Symbol> = Vec::with_capacity(row_length);
            row.push(Symbol::Ground);
            row.extend(l.chars().map(Symbol::from));
            row.push(Symbol::Ground);
            row
        }));
        map.push(vec![Symbol::Ground; row_length]);

        let (origin, (origin_next_1, origin_next_2)) = process_origin(&mut map);
        Self {
            map,
            origin,
            origin_next_1,
            origin_next_2,
        }
    }

    // Count the number of points inside the pipe contour that starts at origin
    fn count_points_in_contour(&self) -> usize {
        // Find contour, transform each coordinate to its signed value to avoid overflow on
        // upcoming calculations
        let contour: Vec<SPos> = self
            .find_contour()
            .into_iter()
            .map(|pos| (pos.0 as isize, pos.1 as isize))
            .collect();

        // Find bounding eges
        let (x_range, y_range) = bounding_ranges(&contour);

        // Count points inside the contour, excluding the ones that are on the eges
        let vertices: HashSet<SPos> = HashSet::from_iter(contour.to_owned());
        let mut count = 0;
        for x in x_range {
            for y in y_range.clone() {
                let pos = (x, y);
                if !vertices.contains(&pos) && point_inside_contour(pos, &contour) {
                    count += 1;
                }
            }
        }
        count
    }

    // Find a closed contour of pipes that start at origin
    fn find_contour(&self) -> Vec<Pos> {
        let mut contour = vec![self.origin, self.origin_next_1];
        let mut prev = self.origin;
        let mut curr = self.origin_next_1;
        loop {
            let next = self.find_next(curr, prev);
            if next == self.origin {
                break;
            }
            contour.push(next);
            prev = curr;
            curr = next;
        }
        contour
    }

    // Walk from the two origin neighbors simultaneously counting the number of steps before they meet
    fn walk(&self) -> usize {
        // we already stepped away from origin onto each neighbor
        let mut count = 1;
        let mut prev_1 = self.origin;
        let mut prev_2 = self.origin;
        let mut curr_1 = self.origin_next_1;
        let mut curr_2 = self.origin_next_2;
        loop {
            let next_1 = self.find_next(curr_1, prev_1);
            let next_2 = self.find_next(curr_2, prev_2);
            if next_1 == next_2 {
                break;
            }
            prev_1 = curr_1;
            curr_1 = next_1;
            prev_2 = curr_2;
            curr_2 = next_2;
            count += 1;
        }

        // include the last step that was not made because of the stop condition
        count + 1
    }

    fn get_symbol(&self, pos: Pos) -> Symbol {
        self.map[pos.0][pos.1]
    }

    fn find_next(&self, start: Pos, prev: Pos) -> Pos {
        if let Symbol::Pipe(p) = self.get_symbol(start) {
            return p
                .next_allowed()
                .into_iter()
                .filter_map(|(pp, inc)| {
                    let next_pos = increment(start, inc);
                    match self.get_symbol(next_pos) {
                        Symbol::Pipe(np) if (np == pp && next_pos != prev) => Some(next_pos),
                        _ => None,
                    }
                })
                .collect::<Vec<Pos>>()[0];
        }
        panic!("One should walk along pipes only, found: {prev:?} --> {start:?}");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Start,
    Ground,
    Pipe(PipeType),
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Symbol::Ground,
            'S' => Symbol::Start,
            s => Symbol::Pipe(s.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeType::NorthSouth,
            '-' => PipeType::EastWest,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            _ => panic!("Unrecognized pipe: {value}"),
        }
    }
}

impl PipeType {
    fn next_allowed(&self) -> Vec<(Self, PosInc)> {
        match self {
            PipeType::NorthSouth => vec![
                (PipeType::SouthEast, (-1, 0)),
                (PipeType::SouthWest, (-1, 0)),
                (PipeType::NorthEast, (1, 0)),
                (PipeType::NorthWest, (1, 0)),
                (PipeType::NorthSouth, (-1, 0)),
                (PipeType::NorthSouth, (1, 0)),
            ],
            PipeType::EastWest => vec![
                (PipeType::SouthEast, (0, -1)),
                (PipeType::SouthWest, (0, 1)),
                (PipeType::NorthEast, (0, -1)),
                (PipeType::NorthWest, (0, 1)),
                (PipeType::EastWest, (0, 1)),
                (PipeType::EastWest, (0, -1)),
            ],
            PipeType::NorthEast => {
                vec![
                    (PipeType::NorthSouth, (-1, 0)),
                    (PipeType::SouthEast, (-1, 0)),
                    (PipeType::SouthWest, (-1, 0)),
                    (PipeType::EastWest, (0, 1)),
                    (PipeType::SouthWest, (0, 1)),
                    (PipeType::NorthWest, (0, 1)),
                ]
            }
            PipeType::NorthWest => {
                vec![
                    (PipeType::NorthSouth, (-1, 0)),
                    (PipeType::SouthWest, (-1, 0)),
                    (PipeType::SouthEast, (-1, 0)),
                    (PipeType::EastWest, (0, -1)),
                    (PipeType::NorthEast, (0, -1)),
                    (PipeType::SouthEast, (0, -1)),
                ]
            }
            PipeType::SouthWest => {
                vec![
                    (PipeType::NorthSouth, (1, 0)),
                    (PipeType::NorthWest, (1, 0)),
                    (PipeType::NorthEast, (1, 0)),
                    (PipeType::EastWest, (0, -1)),
                    (PipeType::SouthEast, (0, -1)),
                    (PipeType::NorthEast, (0, -1)),
                ]
            }
            PipeType::SouthEast => {
                vec![
                    (PipeType::NorthSouth, (1, 0)),
                    (PipeType::NorthEast, (1, 0)),
                    (PipeType::NorthWest, (1, 0)),
                    (PipeType::EastWest, (0, 1)),
                    (PipeType::SouthWest, (0, 1)),
                    (PipeType::NorthWest, (0, 1)),
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process_4() {
        let input = r#"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "#;
        assert_eq!(4, process(input));
    }

    #[test]
    fn part2_process_8() {
        let input = r#"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "#;
        assert_eq!(8, process(input));
    }

    #[test]
    fn part2_process_10() {
        let input = r#"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "#;
        assert_eq!(10, process(input));
    }
}
