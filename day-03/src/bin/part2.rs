use regex::{Match, Regex};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> u32 {
    let part_num: Regex = Regex::new(r"[0-9]+").unwrap();
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .collect();

    let mut total = 0;
    for (line_no, row) in lines.iter().enumerate() {
        let prev_row = if line_no > 0 {
            Some(lines[line_no - 1])
        } else {
            None
        };
        let next_row = if line_no < lines.len() - 1 {
            Some(lines[line_no + 1])
        } else {
            None
        };

        total += process_row(row, prev_row, next_row, &part_num);
    }
    total
}

fn process_row(row: &str, prev_row: Option<&str>, next_row: Option<&str>, re: &Regex) -> u32 {
    let mut row_total: u32 = 0;
    for (gear_pos, c) in row.char_indices() {
        if c == '*' {
            let mut part_numbers: Vec<u32> = Vec::new();
            let gear_range_start = if gear_pos > 0 { gear_pos - 1 } else { gear_pos };
            let gear_range_end = if gear_pos < row.len() - 1 {
                gear_pos + 1
            } else {
                gear_pos
            };

            // previous row
            if let Some(pr) = prev_row {
                for m in re.find_iter(pr) {
                    if overlaps_gear_range(m, gear_range_start, gear_range_end) {
                        if let Ok(num) = str::parse::<u32>(m.as_str()) {
                            part_numbers.push(num);
                        }
                    }
                }
            }

            // current row
            for m in re.find_iter(row) {
                if overlaps_gear_range(m, gear_range_start, gear_range_end) {
                    if let Ok(num) = str::parse::<u32>(m.as_str()) {
                        part_numbers.push(num);
                    }
                }
            }

            // next row
            if let Some(nr) = next_row {
                for m in re.find_iter(nr) {
                    if overlaps_gear_range(m, gear_range_start, gear_range_end) {
                        if let Ok(num) = str::parse::<u32>(m.as_str()) {
                            part_numbers.push(num);
                        }
                    }
                }
            }

            // a gear is a '*' that has exactly two adjacent numbers
            if part_numbers.len() == 2 {
                row_total += part_numbers[0] * part_numbers[1];
            }
        }
    }
    row_total
}

fn overlaps_gear_range(m: Match, gear_range_start: usize, gear_range_end: usize) -> bool {
    let match_start = m.start();
    let match_end = m.end() - 1;
    match_start <= gear_range_end && match_end >= gear_range_start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(467835, process(input));
    }
}
