fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
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
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    l.split(',').for_each(|s| {
        // remove lens
        if let Some((label, _)) = s.split_once('-') {
            let box_idx = find_hash(label);
            if let Some(lens_idx) = boxes[box_idx]
                .iter()
                .position(|(curr_label, _)| *curr_label == label)
            {
                boxes[box_idx].remove(lens_idx);
            }
        }

        // add lens
        if let Some((label, f)) = s.split_once('=') {
            let box_idx = find_hash(label);
            let focal_length = f.parse::<usize>().unwrap();
            let lens_record = (label, focal_length);
            if let Some(lens_idx) = boxes[box_idx]
                .iter()
                .position(|(curr_label, _)| *curr_label == label)
            {
                // old lens found - replace
                if let Some(el) = boxes[box_idx].get_mut(lens_idx) {
                    *el = lens_record;
                }
            } else {
                // no old lens - append
                boxes[box_idx].push(lens_record);
            }
        }
    });

    boxes.iter().enumerate().fold(0, |acc, (box_idx, b)| {
        acc + b
            .iter()
            .enumerate()
            .fold(0, |box_acc, (lens_idx, (_, focal_length))| {
                box_acc + focal_length * (lens_idx + 1) * (box_idx + 1)
            })
    })
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
    fn part2_process() {
        let input = r#"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "#;
        assert_eq!(145, process(input));
    }
}
