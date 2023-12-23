use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (usize, usize);
type NodeTree = HashMap<Pos, Vec<(Pos, usize)>>;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    assert_eq!(6630, answer);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed.chars().collect()),
            _ => None,
        })
        .collect();

    let start_x: usize = 0;
    let start_y: usize = map[start_x]
        .iter()
        .position(|c| *c == '.')
        .expect("failed to find start position");
    let start_pos = (0, start_y);

    let end_x = map.len() - 1;
    let end_y: usize = map[end_x]
        .iter()
        .position(|c| *c == '.')
        .expect("failed to find end position");
    let end_pos = (end_x, end_y);

    let tree = build_node_tree(&map, start_pos, end_pos);
    longest_path(&tree, start_pos, end_pos)
}

fn longest_path(tree: &NodeTree, start_pos: Pos, end_pos: Pos) -> usize {
    let mut max_len = 0;

    let mut stack = Vec::from([(start_pos, 0, HashSet::new())]);
    while let Some((pos, num_steps, mut seen)) = stack.pop() {
        if !seen.insert(pos) {
            continue;
        }

        if pos == end_pos {
            if num_steps > max_len {
                max_len = num_steps;
            }
            continue;
        }

        let adj_list = tree.get(&pos);
        for (next_pos, num_steps_inc) in adj_list.unwrap() {
            stack.push((*next_pos, num_steps + *num_steps_inc, seen.clone()));
        }
    }

    max_len
}

fn build_node_tree(map: &[Vec<char>], start_pos: Pos, end_pos: Pos) -> NodeTree {
    // find all significant vertices: start, end, and all forks
    let mut vertices = HashSet::from([start_pos, end_pos]);
    map.iter().enumerate().for_each(|(pos_x, row)| {
        row.iter().enumerate().for_each(|(pos_y, c)| {
            if *c != '#' {
                let pos = (pos_x, pos_y);
                let neighbors = find_neighbors(pos, map);
                if neighbors.len() > 2 {
                    // we have found a fork
                    vertices.insert(pos);
                }
            }
        });
    });

    // build a tree of significant nodes, replacing each straight segment with a pair: (node, num_steps)
    let mut tree: HashMap<Pos, Vec<(Pos, usize)>> = HashMap::new();
    vertices.iter().for_each(|v_pos| {
        let adj_list = tree.entry(*v_pos).or_default();
        let mut seen: HashSet<Pos> = HashSet::new();

        let mut heap = VecDeque::from([(*v_pos, 0)]);
        while let Some((pos, num_steps)) = heap.pop_front() {
            // allow discovering multiple paths that pass through the node
            if pos != *v_pos && vertices.contains(&pos) {
                adj_list.push((pos, num_steps));
                continue;
            }

            // prevent backtracking
            if !seen.insert(pos) {
                continue;
            }

            for next_pos in find_neighbors(pos, map) {
                heap.push_back((next_pos, num_steps + 1));
            }
        }
    });

    tree
}

fn find_neighbors(curr_pos: Pos, map: &[Vec<char>]) -> Vec<Pos> {
    let num_rows = map.len();
    let num_cols = map.first().unwrap().len();
    let (curr_x, curr_y) = curr_pos;

    let mut next_positions: Vec<Pos> = Vec::new();
    if curr_x > 0 && map[curr_x - 1][curr_y] != '#' {
        next_positions.push((curr_x - 1, curr_y));
    }

    if curr_x < num_rows - 1 && map[curr_x + 1][curr_y] != '#' {
        next_positions.push((curr_x + 1, curr_y));
    }

    if curr_y > 0 && map[curr_x][curr_y - 1] != '#' {
        next_positions.push((curr_x, curr_y - 1));
    }

    if curr_y < num_cols - 1 && map[curr_x][curr_y + 1] != '#' {
        next_positions.push((curr_x, curr_y + 1));
    }

    next_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process_simple() {
        let input = r#"
            #.####
            #....#
            ##.#.#
            #....#
            #.####
        "#;
        assert_eq!(10, process(input));
    }

    #[test]
    fn part2_process_sample() {
        let input = r#"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        "#;
        assert_eq!(154, process(input));
    }
}
