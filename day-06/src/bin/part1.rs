fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let mut records: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(get_records(trimmed)),
            _ => None,
        })
        .collect();
    let distance = records.pop().expect("No distance records found");
    let time = records.pop().expect("No time records found");

    time.iter()
        .zip(distance.iter())
        .map(|(t, d)| find_num_ways_to_beat_race(*t, *d))
        .product()
}

fn find_num_ways_to_beat_race(time: usize, distance: usize) -> usize {
    let half_time: f64 = time as f64 / 2.0;
    let d: f64 = (half_time * half_time - (distance as f64)).sqrt();
    let mut t1: usize = (half_time - d).floor() as usize;
    let mut t2: usize = (half_time + d).ceil() as usize;

    // verify/adjust the bounds
    while t1 * (time - t1) <= distance {
        t1 += 1;
    }

    while t2 * (time - t2) <= distance {
        t2 -= 1;
    }
    t2 - t1 + 1
}

fn get_records(row: &str) -> Vec<usize> {
    let nums = row
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter_map(|n| match n.trim() {
            n_tr if !n_tr.is_empty() => Some(n_tr.parse::<usize>().unwrap()),
            _ => None,
        })
        .collect();
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            Time:      7  15   30
            Distance:  9  40  200
        "#;
        assert_eq!(288, process(input));
    }
}
