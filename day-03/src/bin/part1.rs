use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
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
    let mut row_total = 0;
    for m in re.find_iter(row) {
        let match_start = m.start();
        let match_end = m.end() - 1;

        let adjacence_range_start = if match_start > 0 {
            match_start - 1
        } else {
            match_start
        };
        let adjacence_range_end = if match_end < row.len() - 1 {
            match_end + 1
        } else {
            match_end
        };

        // previous row
        if let Some(pr) = prev_row {
            if pr[adjacence_range_start..=adjacence_range_end]
                .chars()
                .any(is_symbol)
            {
                if let Ok(num) = str::parse::<u32>(m.as_str()) {
                    row_total += num;
                    continue;
                }
            }
        }

        // current row
        if row[adjacence_range_start..=adjacence_range_end]
            .chars()
            .any(is_symbol)
        {
            if let Ok(num) = str::parse::<u32>(m.as_str()) {
                row_total += num;
                continue;
            }
        }

        // next row
        if let Some(nr) = next_row {
            if nr[adjacence_range_start..=adjacence_range_end]
                .chars()
                .any(is_symbol)
            {
                if let Ok(num) = str::parse::<u32>(m.as_str()) {
                    row_total += num;
                }
            }
        }
    }
    row_total
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
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
        assert_eq!(4361, process(input));
    }
}
