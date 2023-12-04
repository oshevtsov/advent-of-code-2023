use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .collect();

    // use BTreeMap here to be able to iterate in order
    let mut cards: BTreeMap<u32, usize> = BTreeMap::new();
    for line in lines {
        if let Some((card_and_num, lists)) = line.split_once(':') {
            if let Some((_, id)) = card_and_num.split_once(' ') {
                let card_id = id
                    .trim_start()
                    .parse::<u32>()
                    .expect("failed to parse card number");

                let num_common = cards.entry(card_id).or_default();
                if let Some((winning_input, our_input)) = lists.split_once('|') {
                    let winning = parse_list(winning_input);
                    let our = parse_list(our_input);
                    *num_common = our.intersection(&winning).count();
                }
            }
        }
    }
    count_cards(cards)
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

fn count_cards(cards: BTreeMap<u32, usize>) -> u32 {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for (id, num_common) in cards.iter() {
        let count = *counts.entry(*id).and_modify(|curr| *curr += 1).or_insert(1);
        for i in 1..=*num_common {
            let copy_id = id + (i as u32);
            counts
                .entry(copy_id)
                .and_modify(|curr| *curr += count)
                .or_insert(count);
        }
    }
    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(30, process(input));
    }
}
