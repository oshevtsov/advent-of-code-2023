use std::{cmp, collections::VecDeque};

type Terrain = Vec<Vec<char>>;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let lines = to_lines(input);

    // Parse input into a sequence of terrains
    let terrains = parse_lines(lines);

    terrains
        .iter()
        .fold(0, |total, terrain| total + process_terrain(terrain))
}

fn to_lines(input: &str) -> VecDeque<&str> {
    let mut lines: VecDeque<&str> = input.lines().map(|l| l.trim()).collect();

    // Get rid of the empty lines at the beginning
    while let Some(l) = lines.front() {
        if !l.is_empty() {
            break;
        }
        lines.pop_front();
    }

    lines
}

fn parse_lines(mut lines: VecDeque<&str>) -> Vec<Terrain> {
    let mut result = Vec::new();

    let mut t: Terrain = Vec::new();
    while let Some(l) = lines.pop_front() {
        if l.is_empty() {
            result.push(t);
            t = Vec::new();
            continue;
        }

        t.push(l.chars().collect());
    }

    // do not forget the last terrain at the end of the input
    if !t.is_empty() {
        result.push(t);
    }
    result
}

fn process_terrain(terrain: &Terrain) -> usize {
    let mut total = 0;

    total += find_horizontal_reflection(terrain)
        .into_iter()
        .fold(0, |acc, rows_before| acc + 100 * rows_before);

    total += find_vertical_reflection(terrain).into_iter().sum::<usize>();

    total
}

fn find_horizontal_reflection(terrain: &Terrain) -> Vec<usize> {
    let mut result = Vec::new();

    let num_rows = terrain.len();
    for rows_before in 1..=(num_rows - 1) {
        let num_overlap = cmp::min(rows_before, num_rows - rows_before);

        if (0..num_overlap).fold(0, |acc, i| {
            acc + terrain[rows_before - 1 - i]
                .iter()
                .zip(terrain[rows_before + i].iter())
                .filter(|(&c_l, &c_r)| c_l != c_r)
                .count()
        }) == 1
        {
            result.push(rows_before);
        }
    }

    result
}

fn find_vertical_reflection(terrain: &Terrain) -> Vec<usize> {
    let mut result = Vec::new();
    let num_cols = terrain.first().unwrap().len();

    for cols_before in 1..=(num_cols - 1) {
        let num_overlap = cmp::min(cols_before, num_cols - cols_before);

        if (0..num_overlap).fold(0, |acc, j| {
            acc + terrain
                .iter()
                .filter(|l| l[cols_before - 1 - j] != l[cols_before + j])
                .count()
        }) == 1
        {
            result.push(cols_before);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
        let input = r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#;
        assert_eq!(400, process(input));
    }
}
