use std::collections::HashSet;

type Pos = (usize, usize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input, 64);
    println!("Part 1 answer: {answer}");
}

// Make an observation that after the tile has been visited, it will be revisited after two steps
// again and again (step out and step back again). There can be many ways a tile is revisited after
// stepping out (by 1 step or multiple steps, but the parity of each of those variants on a 2D grid
// is the same), but all these variants are counted as one. So, it is enough to find the shortest
// path to every tile and then remember that they should be counted every second step from there
// onward.
fn process(input: &str, num_steps: usize) -> usize {
    let (map, pos) = parse_input(input);

    let mut next_tiles = Vec::new();
    let mut steps_left = num_steps;
    let mut tiles_reached: HashSet<Pos> = HashSet::new();
    let mut visited: HashSet<Pos> = HashSet::from([pos]);

    if num_steps % 2 == 0 {
        // We need to keep track of steps 0,2,4,6,...
        next_tiles.push(pos);
        tiles_reached.insert(pos);
    } else {
        // We need to keep track of steps 1,3,5,7,...
        for next_pos in make_step(pos, &map) {
            next_tiles.push(next_pos);
            tiles_reached.insert(next_pos);
            visited.insert(next_pos);
        }
        steps_left -= 1;
    }

    // Now we can make two steps at a time, since we took into account the parity of steps at the
    // beginning.
    let mut tmp = Vec::new();
    for _ in 1..=steps_left / 2 {
        // first step
        while let Some(curr_pos) = next_tiles.pop() {
            for next_pos in make_step(curr_pos, &map) {
                if !visited.insert(next_pos) {
                    continue;
                }
                tmp.push(next_pos);
            }
        }

        // second step
        while let Some(curr_pos) = tmp.pop() {
            for next_pos in make_step(curr_pos, &map) {
                if !visited.insert(next_pos) {
                    continue;
                }
                tiles_reached.insert(next_pos);
                next_tiles.push(next_pos);
            }
        }
    }

    tiles_reached.len()
}

fn make_step(curr_pos: Pos, map: &[Vec<char>]) -> Vec<Pos> {
    let (curr_x, curr_y) = curr_pos;
    let num_rows = map.len();
    let num_cols = map.first().expect("map is empty").len();

    let mut next_pos: Vec<Pos> = Vec::new();

    // north
    if curr_x > 0 && map[curr_x - 1][curr_y] != '#' {
        next_pos.push((curr_x - 1, curr_y));
    }

    // south
    if curr_x < num_rows - 1 && map[curr_x + 1][curr_y] != '#' {
        next_pos.push((curr_x + 1, curr_y));
    }

    // west
    if curr_y > 0 && map[curr_x][curr_y - 1] != '#' {
        next_pos.push((curr_x, curr_y - 1));
    }

    // east
    if curr_y < num_cols - 1 && map[curr_x][curr_y + 1] != '#' {
        next_pos.push((curr_x, curr_y + 1));
    }

    next_pos
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let mut start_x = 0;
    let mut start_y = 0;

    let map: Vec<Vec<char>> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed.chars().collect()),
            _ => None,
        })
        .collect();

    for (row_idx, row) in map.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == 'S') {
            start_x = row_idx;
            start_y = col_idx;
            break;
        }
    }

    (map, (start_x, start_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "#;
        assert_eq!(16, process(input, 6));
    }
}
