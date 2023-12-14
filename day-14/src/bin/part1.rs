type Platform = Vec<Vec<char>>;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let platform: Platform = parse_input(input);
    let tilted_platform = tilt_north(platform);

    let num_rows = tilted_platform.len();
    tilted_platform
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, row)| {
            acc + row.iter().filter(|&&c| c == 'O').count() * (num_rows - idx)
        })
}

fn parse_input(input: &str) -> Platform {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed.chars().collect()),
            _ => None,
        })
        .collect()
}

fn tilt_north(platform: Platform) -> Platform {
    let mut transposed = transpose(&platform);

    for row in transposed.iter_mut() {
        let mut cur: usize = 0;

        // Move all 'O' as far as possible to the left
        while let Some(pos) = row[cur..].iter().position(|c| *c == 'O') {
            let real_pos = cur + pos;

            if let Some(new_pos) = row[cur..real_pos].iter().rposition(|c| *c == '#') {
                let new_real_pos = cur + new_pos;
                row.swap(real_pos, new_real_pos + 1);
                cur = new_real_pos + 2;
            } else {
                row.swap(real_pos, cur);
                cur += 1;
            }
        }
    }

    transpose(&transposed)
}

fn transpose(platform: &Platform) -> Platform {
    assert!(!platform.is_empty());

    let rows = platform.len();
    let cols = platform[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| platform[row][col]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tilt_platform() {
        let input = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        let platform = parse_input(input);

        let tilted_input = r#"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        "#;
        let tilted_platform = parse_input(tilted_input);
        assert_eq!(tilted_platform, tilt_north(platform));
    }

    #[test]
    fn part1_process() {
        let input = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        assert_eq!(136, process(input));
    }
}
