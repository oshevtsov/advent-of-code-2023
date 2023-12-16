use std::collections::{HashSet, VecDeque};

type Pos = (usize, usize);
type Node = (Pos, Direction);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let layout: Vec<Vec<char>> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed.chars().collect()),
            _ => None,
        })
        .collect();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut moves: VecDeque<Node> = VecDeque::from([((0, 0), Direction::Right)]);

    while let Some(current_node) = moves.pop_front() {
        visited.insert(current_node);
        for next_node in find_next_nodes(current_node, &layout) {
            if visited.insert(next_node) {
                moves.push_back(next_node);
            }
        }
    }
    visited
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<Pos>>()
        .len()
}

fn find_next_nodes(node: Node, layout: &[Vec<char>]) -> Vec<Node> {
    let num_rows = layout.len();
    let num_cols = layout.first().expect("layout is empty").len();

    let ((row, col), dir) = node;

    let mut next_nodes = Vec::with_capacity(2);
    match layout[row][col] {
        '.' => match dir {
            Direction::Left if col > 0 => next_nodes.push(((row, col - 1), dir)),
            Direction::Right if col < num_cols - 1 => next_nodes.push(((row, col + 1), dir)),
            Direction::Up if row > 0 => next_nodes.push(((row - 1, col), dir)),
            Direction::Down if row < num_rows - 1 => next_nodes.push(((row + 1, col), dir)),
            _ => (),
        },
        '/' => match dir {
            Direction::Left if row < num_rows - 1 => {
                next_nodes.push(((row + 1, col), Direction::Down))
            }
            Direction::Right if row > 0 => next_nodes.push(((row - 1, col), Direction::Up)),
            Direction::Up if col < num_cols - 1 => {
                next_nodes.push(((row, col + 1), Direction::Right))
            }
            Direction::Down if col > 0 => next_nodes.push(((row, col - 1), Direction::Left)),
            _ => (),
        },
        '\\' => match dir {
            Direction::Left if row > 0 => next_nodes.push(((row - 1, col), Direction::Up)),
            Direction::Right if row < num_rows - 1 => {
                next_nodes.push(((row + 1, col), Direction::Down))
            }
            Direction::Up if col > 0 => next_nodes.push(((row, col - 1), Direction::Left)),
            Direction::Down if col < num_cols - 1 => {
                next_nodes.push(((row, col + 1), Direction::Right))
            }
            _ => (),
        },
        '-' => match dir {
            Direction::Left if col > 0 => next_nodes.push(((row, col - 1), dir)),
            Direction::Right if col < num_cols - 1 => next_nodes.push(((row, col + 1), dir)),
            Direction::Up | Direction::Down => {
                if col > 0 {
                    next_nodes.push(((row, col - 1), Direction::Left));
                }

                if col < num_cols - 1 {
                    next_nodes.push(((row, col + 1), Direction::Right));
                }
            }
            _ => (),
        },
        '|' => match dir {
            Direction::Left | Direction::Right => {
                if row > 0 {
                    next_nodes.push(((row - 1, col), Direction::Up))
                }

                if row < num_rows - 1 {
                    next_nodes.push(((row + 1, col), Direction::Down));
                }
            }
            Direction::Up if row > 0 => next_nodes.push(((row - 1, col), dir)),
            Direction::Down if row < num_rows - 1 => next_nodes.push(((row + 1, col), dir)),
            _ => (),
        },
        _ => (),
    }
    next_nodes
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#;
        assert_eq!(46, process(input));
    }
}
