use std::cmp;

type Pos = (usize, usize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input, 1_000_000);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str, expansion_factor: usize) -> usize {
    let map = input_to_map(input);
    let galaxies: Vec<Pos> = find_galaxies(&map, expansion_factor);

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

fn find_galaxies(map: &[Vec<char>], expansion_factor: usize) -> Vec<Pos> {
    let mut galaxies: Vec<Pos> = Vec::new();
    let empty_rows = find_empty_rows(map);
    let empty_cols = find_empty_cols(map);

    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, c)| {
            if *c == '#' {
                let num_empty_rows_before = empty_rows.iter().filter(|&idx| *idx < row_idx).count();
                let num_empty_cols_before = empty_cols.iter().filter(|&idx| *idx < col_idx).count();
                let expanded_row = row_idx + num_empty_rows_before * (expansion_factor - 1);
                let expanded_col = col_idx + num_empty_cols_before * (expansion_factor - 1);
                galaxies.push((expanded_row, expanded_col));
            }
        })
    });

    galaxies
}

fn find_empty_rows(map: &[Vec<char>]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            if row.iter().any(|&c| c == '#') {
                return None;
            }
            Some(row_idx)
        })
        .collect()
}

fn find_empty_cols(map: &[Vec<char>]) -> Vec<usize> {
    let mut empty_cols: Vec<usize> = Vec::new();
    let row_len = map.first().unwrap().len();
    for col_idx in 0..row_len {
        if map.iter().all(|row| row[col_idx] == '.') {
            empty_cols.push(col_idx);
        }
    }
    empty_cols
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
    fn part2_process() {
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
        assert_eq!(374, process(input, 2));
        assert_eq!(1030, process(input, 10));
        assert_eq!(8410, process(input, 100));
    }
}
