fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

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

fn process(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(process_row)
        .sum()
}

pub fn process_row(row: &str) -> u32 {
    let first_digit = find_first_digit(row);
    let last_digit = find_last_digit(row);
    format!("{first_digit}{last_digit}")
        .parse()
        .expect("Failed to parse a two-digit number")
}

fn find_first_digit(row: &str) -> char {
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

fn find_last_digit(row: &str) -> char {
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

    #[test]
    fn part2_contains_digits_and_words() {
        let row = "two1nine";
        assert_eq!('2', find_first_digit(row));
        assert_eq!('9', find_last_digit(row));
        assert_eq!(29, process_row(row));
    }

    #[test]
    fn part2_contains_more_than_two_words() {
        let row = "eightwothree";
        assert_eq!('8', find_first_digit(row));
        assert_eq!('3', find_last_digit(row));
        assert_eq!(83, process_row(row));
    }

    #[test]
    fn part2_contains_words_digits_and_noise() {
        let row = "abcone2threexyz";
        assert_eq!('1', find_first_digit(row));
        assert_eq!('3', find_last_digit(row));
        assert_eq!(13, process_row(row));
    }

    #[test]
    fn part2_contains_sixteen() {
        let row = "7pqrstsixteen";
        assert_eq!('7', find_first_digit(row));
        assert_eq!('6', find_last_digit(row));
        assert_eq!(76, process_row(row));
    }

    #[test]
    fn part2_process() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(281, process(input));
    }
}
