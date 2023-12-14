use std::collections::HashMap;

type Platform = Vec<Vec<char>>;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> usize {
    let platform: Platform = parse_input(input);
    let cycled_platform = cycle_tilt(platform, 1_000_000_000);

    let num_rows = cycled_platform.len();
    cycled_platform
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, row)| {
            acc + row.iter().filter(|&&c| c == 'O').count() * (num_rows - idx)
        })
}

fn parse_input(input: &str) -> Platform {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed.chars().collect()),
            _ => None,
        })
        .collect()
}

fn cycle_tilt(platform: Platform, cycles: usize) -> Platform {
    // Since we have to make many cycles, it is quite probable that we will get stuck in a loop
    // switching between a limited number of states. Therefore, we explore the state space, caching
    // all the transitions we encounter: (prev, curr) -> num_cycles.
    let mut stats: HashMap<(String, String), usize> = HashMap::new();
    let mut prev = platform;

    let mut last: Platform = prev.clone();
    let mut last_step: usize = 0;
    for i in 1..=cycles {
        last_step = i;
        let curr = make_cycle(prev.clone());

        let key = (serialize(&prev), serialize(&curr));
        if stats.contains_key(&key) {
            // When we encounter a pair of states we have seen before, we can stop and fast-forward
            // the rest of iterations (see below).
            last = curr;
            break;
        }
        stats.insert(key, i);
        prev = curr;
    }

    // In case we have not encountered any cycles, we just return the latest state (see last line
    // in the loop above).
    if last_step == cycles {
        return prev;
    }

    // At this point, we have found a cycle, so we can fast-forward the calculation. The cycle may
    // not circle back to the initial state, but rather one of the later ones. Order the state
    // transitions we have found by the number of cycles.
    let mut s = stats
        .into_iter()
        .collect::<Vec<((String, String), usize)>>();
    s.sort_by_key(|(_, c)| *c);

    // Find the final configuration given the information we have so far.
    find_final_config(s, (serialize(&prev), serialize(&last)), cycles)
}

fn find_final_config(
    stats_v: Vec<((String, String), usize)>,
    last_key: (String, String),
    cycles: usize,
) -> Platform {
    // Since the cycle may not close onto the initial state, there is a sequence of unique
    // transitions in the beginning that we have to account for first, before using our knowledge
    // about the cycle.
    let mut cycles_left = cycles;
    let mut iterations_before_cycle: usize = 0;
    for i in 0..stats_v.len() - 1 {
        let prev = stats_v.get(i).unwrap();
        let curr = stats_v.get(i + 1).unwrap();

        cycles_left -= curr.1 - prev.1;

        if prev.0 == last_key {
            iterations_before_cycle = i;
            break;
        }
    }

    // After we subtracted the unique transitions, we can fast-forward the cycles until the very
    // end, where there may be a portion of a full cycle left to be explored.
    let period = stats_v.len() - iterations_before_cycle;
    let cycles_left = cycles_left % period;

    // We have exhausted all the cycles, take the last configuration and complete the remaining
    // number of iterations.
    let mut result = parse_input(&last_key.1);
    for _ in 0..cycles_left {
        result = make_cycle(result);
    }

    result
}

fn serialize(platform: &Platform) -> String {
    platform
        .iter()
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn make_cycle(platform: Platform) -> Platform {
    let tilted_north = tilt_north(platform);
    let tilted_west = tilt_west(tilted_north);
    let tilted_south = tilt_south(tilted_west);
    tilt_east(tilted_south)
}

fn tilt_north(platform: Platform) -> Platform {
    let mut transposed = transpose(&platform);

    for row in transposed.iter_mut() {
        move_rocks_left(row);
    }

    transpose(&transposed)
}

fn tilt_west(mut platform: Platform) -> Platform {
    for row in platform.iter_mut() {
        move_rocks_left(row);
    }

    platform
}

fn tilt_south(platform: Platform) -> Platform {
    let mut transposed = transpose(&platform);

    for row in transposed.iter_mut() {
        row.reverse();
        move_rocks_left(row);
        row.reverse();
    }

    transpose(&transposed)
}

fn tilt_east(mut platform: Platform) -> Platform {
    for row in platform.iter_mut() {
        row.reverse();
        move_rocks_left(row);
        row.reverse();
    }

    platform
}

// Helper function to move all rocks in the row as much as possible to the left
fn move_rocks_left(row: &mut [char]) {
    let mut cur: usize = 0;

    // Move all 'O' as far as possible to the left
    while let Some(pos) = row[cur..].iter().position(|c| *c == 'O') {
        let real_pos = cur + pos;

        if let Some(new_pos) = row[cur..real_pos].iter().rposition(|c| *c == '#') {
            let new_real_pos = cur + new_pos;
            row.swap(real_pos, new_real_pos + 1);
            cur = new_real_pos + 2;
        } else {
            row.swap(real_pos, cur);
            cur += 1;
        }
    }
}

fn transpose(platform: &Platform) -> Platform {
    assert!(!platform.is_empty());

    let rows = platform.len();
    let cols = platform[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| platform[row][col]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_make_cycle() {
        let input = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        let platform = parse_input(input);

        let cycled_input = r#"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        "#;
        let cycled_platform = parse_input(cycled_input);
        assert_eq!(cycled_platform, make_cycle(platform));
    }

    #[test]
    fn part2_cycle_tilt_3() {
        let input = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        let platform = parse_input(input);

        let cycled_input = r#"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
        "#;
        let cycled_platform = parse_input(cycled_input);
        // assert_eq!(cycled_platform, cycle_tilt_naive(platform.clone(), 3));
        assert_eq!(cycled_platform, cycle_tilt(platform, 3));
    }

    #[test]
    fn part2_process() {
        let input = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        assert_eq!(64, process(input));
    }
}
