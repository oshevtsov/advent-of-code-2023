use std::collections::VecDeque;

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(process_line(trimmed)),
            _ => None,
        })
        .sum()
}

fn process_line(l: &str) -> usize {
    if let Some((pattern, counts_str)) = l.split_once(' ') {
        let counts: VecDeque<usize> = counts_str
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        return count_arrangements(pattern.to_owned(), counts);
    }
    panic!("Could not parse the line");
}

fn count_arrangements(pattern: String, counts: VecDeque<usize>) -> usize {
    let total_count = counts.iter().sum();
    do_count_arrangements(pattern.trim_matches('.').to_owned(), counts, total_count)
}

fn do_count_arrangements(
    pattern: String,
    mut counts: VecDeque<usize>,
    total_count: usize,
) -> usize {
    if pattern.is_empty() || counts.is_empty() {
        return 0;
    }

    if let Some(count) = counts.pop_front() {
        let variants: Vec<String> = find_clusters(&pattern)
            .into_iter()
            .filter_map(|cluster| {
                let replaced_patterns: Vec<String> =
                    generate_replaced_patterns(&pattern, cluster, count)
                        .into_iter()
                        .filter(|p| {
                            p.chars().filter(|c| *c == '#' || *c == '?').count()
                                >= total_count - count
                        })
                        .collect();

                if replaced_patterns.is_empty() {
                    return None;
                }
                Some(replaced_patterns)
            })
            .flatten()
            .collect();

        let result = if counts.is_empty() {
            variants
                .into_iter()
                .filter(|p| p.chars().filter(|c| *c == '#').count() == total_count - count)
                .count()
        } else {
            variants.into_iter().fold(0, |total, replaced_pattern| {
                total + do_count_arrangements(replaced_pattern, counts.clone(), total_count - count)
            })
        };

        return result;
    }
    panic!("Something went wrong!");
}

fn generate_replaced_patterns(pattern: &str, cluster: Cluster, hash_count: usize) -> Vec<String> {
    let mut replaced_patterns = Vec::new();

    if cluster.length >= hash_count {
        for i in 0..(cluster.length - hash_count + 1) {
            let hash_range_start = cluster.start + i;
            let hash_range_end = cluster.start + i + hash_count;

            if hash_range_start > 0
                && pattern[..hash_range_start]
                    .chars()
                    .filter(|c| *c == '#')
                    .count()
                    != 0
            {
                // second condition here makes sure that we do not skip any cluster of '#'-s, while
                // matching
                continue;
            }

            if hash_range_end < pattern.len() {
                if let Some(after_replacement) = pattern.chars().nth(hash_range_end) {
                    if after_replacement == '#' {
                        // if after matching we discover that the next character is '#', we
                        // overflow the allowed has_count by at least 1
                        continue;
                    }
                }
            }

            let mut replaced_pattern = String::new();
            if hash_range_end < pattern.len() {
                // pass on the leftover string after substitution of has_count '#'-s
                replaced_pattern.push_str(&pattern[hash_range_end + 1..]);
            }
            replaced_patterns.push(replaced_pattern);
        }
    }

    replaced_patterns
}

#[derive(Debug, Clone, Copy)]
struct Cluster {
    start: usize,
    length: usize,
}

fn find_clusters(pattern: &str) -> Vec<Cluster> {
    let mut clusters = Vec::new();
    if !pattern.is_empty() {
        let re = Regex::new(r"[#?]+").unwrap();
        clusters = re
            .find_iter(pattern)
            .map(|m| Cluster {
                start: m.start(),
                length: m.len(),
            })
            .collect();
    }

    clusters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process_line_1() {
        let l = "???.### 1,1,3";
        assert_eq!(1, process_line(l));
    }

    #[test]
    fn part1_process_line_2() {
        let l = "..?????#?? 4,1";
        assert_eq!(2, process_line(l));
    }

    #[test]
    fn part1_process() {
        let input = r#"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "#;
        assert_eq!(21, process(input));
    }
}
