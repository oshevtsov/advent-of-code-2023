use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    usize,
};

type Pos = (usize, usize);

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> usize {
    let layout: Vec<Vec<usize>> = parse_input(input);

    let num_rows = layout.len();
    let num_cols = layout.first().expect("layout is empty").len();
    let start: Pos = (0, 0);
    let end: Pos = (num_rows - 1, num_cols - 1);

    least_heat_loss(&layout, start, end)
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(
                trimmed
                    .chars()
                    .map(|c| c.to_digit(10).expect("failed to parse digit") as usize)
                    .collect(),
            ),
            _ => None,
        })
        .collect()
}

fn least_heat_loss(layout: &[Vec<usize>], start: Pos, end: Pos) -> usize {
    let mut smallest_loss: HashMap<Node, usize> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    // Initialize the search at the start
    let start_node = Node {
        pos: start,
        dir_history: VecDeque::new(),
    };
    smallest_loss.insert(start_node.clone(), 0);
    heap.push(State {
        heat_loss: 0,
        node: start_node,
    });

    while let Some(State {
        heat_loss: curr_heat_loss,
        node: curr_node,
    }) = heap.pop()
    {
        if curr_node.pos == end {
            if let Some(last_dir) = curr_node.dir_history.back() {
                let can_stop = curr_node
                    .dir_history
                    .iter()
                    .rev()
                    .take(4)
                    .filter(|&dir| dir == last_dir)
                    .count()
                    == 4;
                if can_stop {
                    return curr_heat_loss;
                }
                continue;
            }
        }

        if let Some(current_smallest_loss) = smallest_loss.get(&curr_node) {
            if curr_heat_loss > *current_smallest_loss {
                // We have seen a better path towards this node, skip
                continue;
            }
        }

        for next_node in curr_node.find_next_nodes(layout) {
            let (row, col) = next_node.pos;
            let next_state = State {
                heat_loss: curr_heat_loss + layout[row][col],
                node: next_node,
            };

            if let Some(current_smallest_loss) = smallest_loss.get_mut(&next_state.node) {
                if next_state.heat_loss < *current_smallest_loss {
                    *current_smallest_loss = next_state.heat_loss;
                    heap.push(next_state);
                }
            } else {
                smallest_loss.insert(next_state.node.clone(), next_state.heat_loss);
                heap.push(next_state);
            }
        }
    }

    panic!("Failed to find least heat loss path");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Pos,
    dir_history: VecDeque<Direction>,
}

impl Node {
    // Given the current position and direction, find the possible next nodes that respect the
    // problem constraints:
    // - can move in one direction max 3 times in a row
    // - can turn left or right (with respect to the direction of motion), but not backwards
    fn find_next_nodes(&self, layout: &[Vec<usize>]) -> Vec<Node> {
        let mut next_nodes = Vec::new();

        let num_rows = layout.len();
        let num_cols = layout.first().expect("layout is empty").len();
        let (row, col) = self.pos;

        if let Some(last_dir) = self.dir_history.back() {
            // We keep a history of at most 10 last moves
            let mut next_history = self.dir_history.clone();
            if next_history.len() > 9 {
                next_history.pop_front();
            }

            // Check if we can continue to go straight from the current position
            let can_go_straight = self
                .dir_history
                .iter()
                .filter(|&dir| dir == last_dir)
                .count()
                < 10;

            // Check if we can turn
            let can_turn = self
                .dir_history
                .iter()
                .rev()
                .take(4)
                .filter(|&dir| dir == last_dir)
                .count()
                == 4;

            match last_dir {
                Direction::Left => {
                    // move straight
                    if can_go_straight && col > 0 {
                        let mut dir_history = next_history.clone();
                        dir_history.push_back(*last_dir);
                        next_nodes.push(Node {
                            pos: (row, col - 1),
                            dir_history,
                        });
                    }

                    if can_turn {
                        // move left
                        if row < num_rows - 1 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Down);
                            next_nodes.push(Node {
                                pos: (row + 1, col),
                                dir_history,
                            });
                        }

                        // move right
                        if row > 0 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Up);
                            next_nodes.push(Node {
                                pos: (row - 1, col),
                                dir_history,
                            });
                        }
                    }
                }
                Direction::Right => {
                    // move straight
                    if can_go_straight && col < num_cols - 1 {
                        let mut dir_history = next_history.clone();
                        dir_history.push_back(*last_dir);
                        next_nodes.push(Node {
                            pos: (row, col + 1),
                            dir_history,
                        });
                    }

                    if can_turn {
                        // move left
                        if row > 0 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Up);
                            next_nodes.push(Node {
                                pos: (row - 1, col),
                                dir_history,
                            });
                        }

                        // move right
                        if row < num_rows - 1 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Down);
                            next_nodes.push(Node {
                                pos: (row + 1, col),
                                dir_history,
                            });
                        }
                    }
                }
                Direction::Up => {
                    // move straight
                    if can_go_straight && row > 0 {
                        let mut dir_history = next_history.clone();
                        dir_history.push_back(*last_dir);
                        next_nodes.push(Node {
                            pos: (row - 1, col),
                            dir_history,
                        });
                    }

                    if can_turn {
                        // move left
                        if col > 0 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Left);
                            next_nodes.push(Node {
                                pos: (row, col - 1),
                                dir_history,
                            });
                        }

                        // move right
                        if col < num_cols - 1 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Right);
                            next_nodes.push(Node {
                                pos: (row, col + 1),
                                dir_history,
                            });
                        }
                    }
                }
                Direction::Down => {
                    // move straight
                    if can_go_straight && row < num_rows - 1 {
                        let mut dir_history = next_history.clone();
                        dir_history.push_back(*last_dir);
                        next_nodes.push(Node {
                            pos: (row + 1, col),
                            dir_history,
                        });
                    }

                    if can_turn {
                        // move left
                        if col < num_cols - 1 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Right);
                            next_nodes.push(Node {
                                pos: (row, col + 1),
                                dir_history,
                            });
                        }

                        // move right
                        if col > 0 {
                            let mut dir_history = next_history.clone();
                            dir_history.push_back(Direction::Left);
                            next_nodes.push(Node {
                                pos: (row, col - 1),
                                dir_history,
                            });
                        }
                    }
                }
            }
        } else {
            // We do not have any preferred direction (we are at the beginning of our search), try
            // to go in all possible directions
            if row > 0 {
                next_nodes.push(Node {
                    pos: (row - 1, col),
                    dir_history: VecDeque::from([Direction::Up]),
                });
            }

            if row < num_rows - 1 {
                next_nodes.push(Node {
                    pos: (row + 1, col),
                    dir_history: VecDeque::from([Direction::Down]),
                });
            }

            if col > 0 {
                next_nodes.push(Node {
                    pos: (row, col - 1),
                    dir_history: VecDeque::from([Direction::Left]),
                });
            }

            if col < num_cols - 1 {
                next_nodes.push(Node {
                    pos: (row, col + 1),
                    dir_history: VecDeque::from([Direction::Right]),
                });
            }
        }

        next_nodes
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    heat_loss: usize,
    node: Node,
}

// Implement 'Ord' so that comparison operation chooses smallest based on heat_loss (used for min
// heap implementation)
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.node.pos.cmp(&other.node.pos))
    }
}

// This must be implemented as a requirement of 'Ord', can reuse the above implementation
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process_1() {
        let input = r#"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "#;
        assert_eq!(71, process(input));
    }

    #[test]
    fn part2_process_2() {
        let input = r#"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "#;
        assert_eq!(94, process(input));
    }
}
