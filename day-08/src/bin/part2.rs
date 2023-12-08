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
        // If the starting nodes have to converge to all ending nodes simultaneously, given the
        // problem structure (see `walk_from` below), we need to calculate the smaller common
        // multiple.
        //
        // Note: This problem is impossible to solve for a general graph with an arbitrary array of
        // steps, since there will be no guarantee that such a solution even exists. Therefore,
        // this particular problem has the steps array and the input graph designed in such a way
        // that they conspire to make this solution possible. However, it is impossible to write a
        // completely general algorithm for this.
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

    fn walk_from(&self, origin: String) -> usize {
        let mut step_count: usize = 0;
        let mut start = origin.clone();
        let mut current = start.clone();
        let it = self.steps.iter().cycle();

        // This is the map of all distinct ways to reach a node that ends with a 'Z' starting from
        // a node that ends with an 'A', provided we cycle through the steps indefinitely.
        // Here:
        // key - node we begin from (ends with an 'A' or an intermediate node that ends with a 'Z')
        // value - node that ends with a 'Z' together with the number of steps to reach it from the `key`-node
        let mut count_map: HashMap<String, (String, usize)> = HashMap::new();

        for next_move in it {
            if current.ends_with('Z') && step_count > 0 {
                if count_map.contains_key(&start) {
                    break;
                }

                count_map.insert(start.clone(), (current.clone(), step_count));
                step_count = 0;
                start = current.clone();
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

        // Exploring the input graph structure, we find the following:
        // {"AAA": ("ZZZ", 19637), "ZZZ": ("ZZZ", 19637)}
        // {"LBA": ("NPZ", 11567), "NPZ": ("NPZ", 11567)}
        // {"QGA": ("GHZ", 14257), "GHZ": ("GHZ", 14257)}
        // {"LHA": ("HVZ", 15871), "HVZ": ("HVZ", 15871)}
        // {"XCA": ("NNZ", 19099), "NNZ": ("NNZ", 19099)}
        // {"GSA": ("SPZ", 12643), "SPZ": ("SPZ", 12643)}
        //
        // This means that after reaching the first 'Z'-node, there is a cycle that happens to have
        // same period (number of steps) as the number of steps needed to reach from the 'A'-node
        // to the first 'Z'-node. In other words, to reach a 'Z'-node from a given 'A'-node it is
        // required:
        // "AAA": 19637 * a steps
        // "LBA": 11567 * b steps
        // "QGA": 14257 * c steps
        // "LHA": 15871 * d steps
        // "XCA": 19099 * e steps
        // "GSA": 12643 * f steps
        // where a,b,c,d,e,f are non-zero integers.
        //
        // So, to find the number of steps when all of these periods align, we simply have to find
        // the smaller common period - smallest common multiple
        //
        // P.S. Fun fact is that all of these periods are primes numbers times the length of the
        // steps array (you may verify it yourself).
        println!("{count_map:?}");
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
