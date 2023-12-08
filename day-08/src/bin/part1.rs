use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let lines: VecDeque<&str> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .collect();

    let navigation_map = NavigationMap::from_input(lines);
    navigation_map.walk()
}

#[derive(Debug)]
struct NavigationMap {
    steps: Vec<char>,
    map: HashMap<String, (String, String)>,
}

impl NavigationMap {
    fn from_input(mut lines: VecDeque<&str>) -> Self {
        let steps = lines.pop_front().expect("no steps found").chars().collect();
        let mut map = HashMap::new();
        while let Some(map_row) = lines.pop_front() {
            let (key, value_tuple) = map_row.split_once(" = ").expect("failed to parse map row");
            let (left, right) = value_tuple[1..=8]
                .split_once(", ")
                .expect("failed to parse tuple");
            map.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
        }

        Self { steps, map }
    }

    fn walk(&self) -> usize {
        let mut step_count: usize = 0;
        let mut current = "AAA".to_owned();
        let end = "ZZZ".to_owned();
        let it = self.steps.iter().cycle();
        for next_move in it {
            if current == end {
                break;
            }

            let (left, right) = self
                .map
                .get(&current)
                .expect("failed to find map direction");
            if *next_move == 'L' {
                current = left.to_owned();
            } else {
                current = right.to_owned();
            }

            step_count += 1;
        }
        step_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process_two_steps() {
        let input = r#"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(2, process(input));
    }

    #[test]
    fn part1_process_six_steps() {
        let input = r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(6, process(input));
    }
}
