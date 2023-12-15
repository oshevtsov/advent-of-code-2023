fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .collect();

    lines.iter().fold(0, |acc, l| acc + process_line(l))
}

fn process_line(l: &str) -> usize {
    l.split(',').fold(0, |acc, s| acc + find_hash(s))
}

fn find_hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |acc, b| ((acc + (*b as usize)) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_find_hash() {
        assert_eq!(52, find_hash("HASH"));
    }

    #[test]
    fn part1_process() {
        let input = r#"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "#;
        assert_eq!(1320, process(input));
    }
}
