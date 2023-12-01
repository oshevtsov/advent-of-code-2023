// ----------------------------------------------------------------
// Part 1
// ----------------------------------------------------------------
pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(process_row)
        .sum()
}

fn process_row(row: &str) -> u32 {
    let first_digit = find_first_digit(row);
    let last_digit = find_last_digit(row);
    format!("{first_digit}{last_digit}")
        .parse()
        .expect("Failed to parse a two-digit number")
}

fn find_first_digit(row: &str) -> char {
    let digit_idx = row
        .find(|c: char| c.is_ascii_digit())
        .expect("No digits found in row");
    let digit = row.as_bytes()[digit_idx].into();

    if digit == '0' {
        panic!("Cannot have '0' as the first digit in a two-digit number");
    }

    digit
}

fn find_last_digit(row: &str) -> char {
    let digit_idx = row
        .rfind(|c: char| c.is_ascii_digit())
        .expect("No digits found in row");
    row.as_bytes()[digit_idx].into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_row_two_digits_at_edges() {
        let row = "1abc2";
        assert_eq!('1', find_first_digit(row));
        assert_eq!('2', find_last_digit(row));
        assert_eq!(12, process_row(row));
    }

    #[test]
    fn test_process_row_two_digits_inside() {
        let row = "pqr3stu8vwx";
        assert_eq!('3', find_first_digit(row));
        assert_eq!('8', find_last_digit(row));
        assert_eq!(38, process_row(row));
    }

    #[test]
    fn test_process_row_more_than_two() {
        let row = "a1b2c3d4e5f";
        assert_eq!('1', find_first_digit(row));
        assert_eq!('5', find_last_digit(row));
        assert_eq!(15, process_row(row));
    }

    #[test]
    fn test_process_row_one_digit() {
        let row = "treb7uchet";
        assert_eq!('7', find_first_digit(row));
        assert_eq!('7', find_last_digit(row));
        assert_eq!(77, process_row(row));
    }

    #[test]
    fn test_process_part1() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(142, process_part1(input));
    }
}
