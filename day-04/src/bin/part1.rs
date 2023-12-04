use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .collect();

    let mut total: u32 = 0;
    for line in lines {
        if let Some((_, lists)) = line.split_once(':') {
            if let Some((winning_input, our_input)) = lists.split_once('|') {
                let winning = parse_list(winning_input);
                let our = parse_list(our_input);
                let num_common = our.intersection(&winning).count();

                // every new match doubles the score
                if num_common > 0 {
                    total += 2u32
                        .checked_pow(num_common as u32 - 1)
                        .expect("failed to raise to pow");
                }
            }
        }
    }
    total
}

fn parse_list(input: &str) -> HashSet<u32> {
    input
        .split(' ')
        .filter_map(|s| match s.trim() {
            num if !num.is_empty() => Some(num.parse::<u32>().expect("failed to parse number")),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(13, process(input));
    }
}
