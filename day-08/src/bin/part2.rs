use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
    assert_eq!(8811050362409, answer);
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
        // if the starting nodes have to converge to all ending nodes simultaneously,
        // then the ultimate count is the smallest common multiple of the counts for
        // reaching an end node from the given start node
        self.map
            .keys()
            .filter_map(|key| {
                if key.ends_with('A') {
                    Some(key.to_owned())
                } else {
                    None
                }
            })
            .map(|start| self.walk_from(start))
            .reduce(num::integer::lcm)
            .expect("failed to find count")
    }

    fn walk_from(&self, start: String) -> usize {
        let mut step_count: usize = 0;
        let mut current = start;
        let it = self.steps.iter().cycle();
        for next_move in it {
            if current.ends_with('Z') {
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
    fn part2_process() {
        let input = r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "#;
        assert_eq!(6, process(input));
    }
}
