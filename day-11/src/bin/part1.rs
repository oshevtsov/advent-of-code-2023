use std::cmp;

type Pos = (usize, usize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let original_map = input_to_map(input);
    let map = expand_map(original_map);
    let galaxies: Vec<Pos> = find_galaxies(&map);

    galaxies.iter().enumerate().fold(0, |total, (idx, start)| {
        let subtotal = galaxies
            .iter()
            .take(galaxies.len())
            .skip(idx + 1)
            .fold(0, |acc, end| acc + calc_manhattan_distance(*start, *end));
        total + subtotal
    })
}

fn input_to_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed.chars().collect()),
            _ => None,
        })
        .collect()
}

fn calc_manhattan_distance(start: Pos, end: Pos) -> usize {
    let (s_row, s_col) = start;
    let (e_row, e_col) = end;

    let distance_row = cmp::max(s_row, e_row) - cmp::min(s_row, e_row);
    let distance_col = cmp::max(s_col, e_col) - cmp::min(s_col, e_col);
    distance_col + distance_row
}

fn find_galaxies(map: &[Vec<char>]) -> Vec<Pos> {
    let mut galaxies: Vec<Pos> = Vec::new();

    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, c)| {
            if *c == '#' {
                galaxies.push((row_idx, col_idx));
            }
        })
    });

    galaxies
}

fn expand_map(original_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = original_map;

    // expand rows
    let mut empty_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            if row.iter().any(|&c| c == '#') {
                return None;
            }
            Some(row_idx)
        })
        .collect();

    empty_rows
        .iter_mut()
        .zip(0..map.len())
        .for_each(|(row_idx, offset)| *row_idx += offset);

    for row_idx in empty_rows {
        let row = map[row_idx].clone();
        map.insert(row_idx, row);
    }

    // expand cols
    let mut empty_cols: Vec<usize> = Vec::new();
    let row_len = map.first().unwrap().len();
    for col_idx in 0..row_len {
        if map.iter().all(|row| row[col_idx] == '.') {
            empty_cols.push(col_idx);
        }
    }

    empty_cols
        .iter_mut()
        .zip(0..row_len)
        .for_each(|(col_idx, offset)| *col_idx += offset);

    for col_idx in empty_cols {
        map.iter_mut().for_each(|row| row.insert(col_idx, '.'));
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_expand_map() {
        let input = r#"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "#;
        let output = r#"
            ....#........
            .........#...
            #............
            .............
            .............
            ........#....
            .#...........
            ............#
            .............
            .............
            .........#...
            #....#.......
        "#;

        let input_map = input_to_map(input);
        let output_map = input_to_map(output);

        assert_eq!(output_map, expand_map(input_map));
    }

    #[test]
    fn part1_process() {
        let input = r#"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "#;
        assert_eq!(374, process(input));
    }
}
