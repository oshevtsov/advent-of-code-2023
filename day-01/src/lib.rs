// ----------------------------------------------------------------
// Part 1
// ----------------------------------------------------------------
pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(process_row_part1)
        .sum()
}

fn process_row_part1(row: &str) -> u32 {
    let first_digit = find_first_digit_part1(row);
    let last_digit = find_last_digit_part1(row);
    format!("{first_digit}{last_digit}")
        .parse()
        .expect("Failed to parse a two-digit number")
}

fn find_first_digit_part1(row: &str) -> char {
    let digit_idx = row
        .find(|c: char| c.is_ascii_digit())
        .expect("No digits found in row");
    let digit = row.as_bytes()[digit_idx].into();

    if digit == '0' {
        panic!("Cannot have '0' as the first digit in a two-digit number");
    }

    digit
}

fn find_last_digit_part1(row: &str) -> char {
    let digit_idx = row
        .rfind(|c: char| c.is_ascii_digit())
        .expect("No digits found in row");
    row.as_bytes()[digit_idx].into()
}

// ----------------------------------------------------------------
// Part 2
// ----------------------------------------------------------------
const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn text_to_digit(text_digit: &str) -> char {
    match text_digit {
        "one" | "1" => '1',
        "two" | "2" => '2',
        "three" | "3" => '3',
        "four" | "4" => '4',
        "five" | "5" => '5',
        "six" | "6" => '6',
        "seven" | "7" => '7',
        "eight" | "8" => '8',
        "nine" | "9" => '9',
        _ => panic!("Cannot translate {text_digit} into a digit"),
    }
}

pub fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(process_row_part2)
        .sum()
}

pub fn process_row_part2(row: &str) -> u32 {
    let first_digit = find_first_digit_part2(row);
    let last_digit = find_last_digit_part2(row);
    format!("{first_digit}{last_digit}")
        .parse()
        .expect("Failed to parse a two-digit number")
}

fn find_first_digit_part2(row: &str) -> char {
    if let Some((idx, length)) = DIGITS
        .iter()
        .filter_map(|d| row.find(d).map(|idx| (idx, d.len())))
        .min_by_key(|(idx, _)| *idx)
    {
        let text_digit = row
            .get(idx..(idx + length))
            .expect("Failed to slice out the text digit");
        return text_to_digit(text_digit);
    }
    panic!("Failed to find the first digit");
}

fn find_last_digit_part2(row: &str) -> char {
    if let Some((idx, length)) = DIGITS
        .iter()
        .filter_map(|d| row.rfind(d).map(|idx| (idx, d.len())))
        .max_by_key(|(idx, _)| *idx)
    {
        let text_digit = row
            .get(idx..(idx + length))
            .expect("Failed to slice out the text digit");
        return text_to_digit(text_digit);
    }
    panic!("Failed to find the last digit");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----------------------------------------------------------------
    // Part 1
    // ----------------------------------------------------------------
    #[test]
    fn test_part1_process_row_two_digits_at_edges() {
        let row = "1abc2";
        assert_eq!('1', find_first_digit_part1(row));
        assert_eq!('2', find_last_digit_part1(row));
        assert_eq!(12, process_row_part1(row));
    }

    #[test]
    fn test_part1_process_row_two_digits_inside() {
        let row = "pqr3stu8vwx";
        assert_eq!('3', find_first_digit_part1(row));
        assert_eq!('8', find_last_digit_part1(row));
        assert_eq!(38, process_row_part1(row));
    }

    #[test]
    fn test_part1_process_row_more_than_two() {
        let row = "a1b2c3d4e5f";
        assert_eq!('1', find_first_digit_part1(row));
        assert_eq!('5', find_last_digit_part1(row));
        assert_eq!(15, process_row_part1(row));
    }

    #[test]
    fn test_part1_process_row_one_digit() {
        let row = "treb7uchet";
        assert_eq!('7', find_first_digit_part1(row));
        assert_eq!('7', find_last_digit_part1(row));
        assert_eq!(77, process_row_part1(row));
    }

    #[test]
    fn test_part1_process_part1() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(142, process_part1(input));
    }

    // ----------------------------------------------------------------
    // Part 2
    // ----------------------------------------------------------------
    #[test]
    fn test_part2_contains_digits_and_words() {
        let row = "two1nine";
        assert_eq!('2', find_first_digit_part2(row));
        assert_eq!('9', find_last_digit_part2(row));
        assert_eq!(29, process_row_part2(row));
    }

    #[test]
    fn test_part2_contains_more_than_two_words() {
        let row = "eightwothree";
        assert_eq!('8', find_first_digit_part2(row));
        assert_eq!('3', find_last_digit_part2(row));
        assert_eq!(83, process_row_part2(row));
    }

    #[test]
    fn test_part2_contains_words_digits_and_noise() {
        let row = "abcone2threexyz";
        assert_eq!('1', find_first_digit_part2(row));
        assert_eq!('3', find_last_digit_part2(row));
        assert_eq!(13, process_row_part2(row));
    }

    #[test]
    fn test_part2_contains_sixteen() {
        let row = "7pqrstsixteen";
        assert_eq!('7', find_first_digit_part2(row));
        assert_eq!('6', find_last_digit_part2(row));
        assert_eq!(76, process_row_part2(row));
    }

    #[test]
    fn test_part2_process_part2() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(281, process_part2(input));
    }
}
