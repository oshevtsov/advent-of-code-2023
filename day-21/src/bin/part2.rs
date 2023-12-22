use std::collections::HashSet;

type Pos = (usize, usize);
type PosSuper = (Pos, isize, isize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input, 26501365);
    println!("Part 2 answer: {answer}");
}

// Make an observation that after the tile has been visited, it will be revisited after two steps
// again and again (step out and step back again). There can be many ways a tile is revisited after
// stepping out (by 1 step or multiple steps, but the parity of each of those variants on a 2D grid
// is the same), but all these variants are counted as one. So, it is enough to find the shortest
// path to every tile and then remember that they should be counted every second step from there
// onward.
//
// NOTE: In Part 2, we have an infinitely repeating grid of maps, and a huge amount of steps to
// take. Solving this with a brute force in reasonable time is hopeless (the solution below would
// eventually cause memory overflow trying to fill the sets of visited tiles). Therefore there must
// be a pattern that repeats. Note that we are now walking on two grids: the original one, and a
// supergrid of maps. Unfortunately, in problems like this, the only way to solve it is to use
// special properties of the input. The input is set up in a special way to make this problem
// solvable. Note that the general algorithm is perfectly fine, but Unfortunately the problem is
// too big to use it.
fn process(input: &str, num_steps: usize) -> usize {
    let (map, pos) = parse_input(input);

    let mut next_tiles: Vec<PosSuper> = Vec::new();
    let mut steps_left = num_steps;
    let mut tiles_reached: HashSet<PosSuper> = HashSet::new();
    let mut visited: HashSet<PosSuper> = HashSet::from([(pos, 0, 0)]);

    if num_steps % 2 == 0 {
        // We need to keep track of steps 0,2,4,6,...
        next_tiles.push((pos, 0, 0));
        tiles_reached.insert((pos, 0, 0));
    } else {
        // We need to keep track of steps 1,3,5,7,...
        for next in make_step_tile_grid(pos, &map) {
            next_tiles.push(next);
            tiles_reached.insert(next);
            visited.insert(next);
        }
        steps_left -= 1;
    }

    // Now we can make two steps at a time, since we took into account the parity of steps at the
    // beginning.
    let mut tmp = Vec::new();
    for _ in 1..=steps_left / 2 {
        // first step
        while let Some((curr_pos, super_x, super_y)) = next_tiles.pop() {
            for (next_pos, super_x_inc, super_y_inc) in make_step_tile_grid(curr_pos, &map) {
                let next = (next_pos, super_x + super_x_inc, super_y + super_y_inc);
                if !visited.insert(next) {
                    continue;
                }
                tmp.push(next);
            }
        }

        // second step
        while let Some((curr_pos, super_x, super_y)) = tmp.pop() {
            for (next_pos, super_x_inc, super_y_inc) in make_step_tile_grid(curr_pos, &map) {
                let next = (next_pos, super_x + super_x_inc, super_y + super_y_inc);
                if !visited.insert(next) {
                    continue;
                }
                tiles_reached.insert(next);
                next_tiles.push(next);
            }
        }
    }

    tiles_reached.len()
}

fn make_step_tile_grid(curr_pos: Pos, map: &[Vec<char>]) -> Vec<(Pos, isize, isize)> {
    let (curr_x, curr_y) = curr_pos;
    let num_rows = map.len();
    let num_cols = map.first().expect("map is empty").len();

    let mut next_pos: Vec<(Pos, isize, isize)> = Vec::new();

    // north
    if curr_x > 0 {
        if map[curr_x - 1][curr_y] != '#' {
            next_pos.push(((curr_x - 1, curr_y), 0, 0));
        }
    } else if map[num_rows - 1][curr_y] != '#' {
        next_pos.push(((num_rows - 1, curr_y), -1, 0));
    }

    // south
    if curr_x < num_rows - 1 {
        if map[curr_x + 1][curr_y] != '#' {
            next_pos.push(((curr_x + 1, curr_y), 0, 0));
        }
    } else if map[0][curr_y] != '#' {
        next_pos.push(((0, curr_y), 1, 0));
    }

    // west
    if curr_y > 0 {
        if map[curr_x][curr_y - 1] != '#' {
            next_pos.push(((curr_x, curr_y - 1), 0, 0));
        }
    } else if map[curr_x][num_cols - 1] != '#' {
        next_pos.push(((curr_x, num_cols - 1), 0, -1));
    }

    // east
    if curr_y < num_cols - 1 {
        if map[curr_x][curr_y + 1] != '#' {
            next_pos.push(((curr_x, curr_y + 1), 0, 0));
        }
    } else if map[curr_x][0] != '#' {
        next_pos.push(((curr_x, 0), 0, 1));
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
    fn part2_process() {
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
        assert_eq!(50, process(input, 10));
        assert_eq!(1594, process(input, 50));
        assert_eq!(6536, process(input, 100));
        assert_eq!(167004, process(input, 500));
        assert_eq!(668697, process(input, 1000));
        assert_eq!(16733044, process(input, 5000));
    }
}
