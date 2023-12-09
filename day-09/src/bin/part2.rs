fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> isize {
    let lines: Vec<Vec<isize>> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => {
                let seq = trimmed
                    .split(' ')
                    .map(|num| num.parse().expect("failed to parse number"))
                    .collect();
                Some(seq)
            }
            _ => None,
        })
        .collect();

    lines.iter().fold(0, |acc, line| acc + process_line(line))
}

fn process_line(line: &[isize]) -> isize {
    let mut sequences: Vec<Vec<isize>> = vec![line.to_owned()];
    while let Some(seq) = sequences.last_mut() {
        if seq.iter().all(|v| *v == 0) {
            seq.insert(0, 0);
            break;
        }

        let next_seq: Vec<isize> = seq.windows(2).map(|w| w[1] - w[0]).collect();
        sequences.push(next_seq);
    }

    let seq_len = sequences.len();
    for i in 0..seq_len - 1 {
        let prev_first = *sequences.get(seq_len - i - 1).unwrap().first().unwrap();
        let curr = sequences.get_mut(seq_len - i - 2).unwrap();
        let curr_new_first = curr.first().unwrap() - prev_first;
        curr.insert(0, curr_new_first);
    }
    *sequences.first().unwrap().first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process_line_1() {
        let line = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(-3, process_line(&line));
    }

    #[test]
    fn part2_process_line_2() {
        let line = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(0, process_line(&line));
    }

    #[test]
    fn part2_process_line_3() {
        let line = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(5, process_line(&line));
    }

    #[test]
    fn part2_process() {
        let input = r#"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "#;
        assert_eq!(2, process(input));
    }
}
