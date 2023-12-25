use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input, 3);
    println!("Part 1 answer: {answer}");
}

// Each node in the input has more than 3 neighbors, so finding 3 edges that will cut the graph
// into two disjoint pieces is a min-cut problem. There are multiple algorithms to solve it, but we
// are using max-flow min-cut theorem, see https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem.
//
// We are not given any source-sink, but we can iterate over pairs of start/end nodes before we encounter the one that
// has max-flow equal to 3 (problem condition). So, we have to focust on start-end min-cut problem.
// To find the min-cut, we first have to find a modified version of the graph, the so-called max-flow residual graph.
// Each pair of nodes have edges connecting them in both directions, and, by default have a capacity of 1, i.e. the edge
// either took part or not in the max flow. When the max-flow residual graph is found, then if we start at the source
// and explore all unsaturated edges, we will explore the first disjoint set of the cut graph. The remainder of the nodes
// will correspondingly be in the co-set.
fn process(input: &str, max_flow: usize) -> usize {
    let components = parse_input(input);

    let ids: Vec<&str> = components.keys().cloned().collect();
    let source = ids.first().unwrap();
    let mut cut_set_size = 0;
    for sink in ids.iter().skip(1) {
        let mut c = components.clone();
        edmonds_karp_flow(&mut c, source, sink);

        let curr_max_flow = c[source]
            .iter()
            .filter(|node| node.residual_capacity() == 0)
            .count();

        if curr_max_flow == max_flow {
            cut_set_size = find_min_cut_set_size(&c, source, sink);
        }
    }

    cut_set_size * (components.len() - cut_set_size)
}

fn find_min_cut_set_size(components: &HashMap<&str, Vec<Node>>, source: &str, sink: &str) -> usize {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut heap: VecDeque<&str> = VecDeque::from([source]);
    while let Some(id) = heap.pop_front() {
        if !visited.insert(id) {
            continue;
        }

        if id == sink {
            break;
        }

        let neighbors = components
            .get(&id)
            .unwrap_or_else(|| panic!("failed to find node {id}"));
        for node in neighbors {
            if node.residual_capacity() > 0 {
                heap.push_back(node.id);
            }
        }
    }

    visited.len()
}

// Implementation of the Edmonds-Karp algorithm, see
// https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm.
fn edmonds_karp_flow(components: &mut HashMap<&str, Vec<Node>>, source: &str, sink: &str) {
    while let Some(bfs_path) = bfs(components, source, sink) {
        update_residual_graph(components, bfs_path);
    }
}

fn update_residual_graph(components: &mut HashMap<&str, Vec<Node>>, bfs_path: Vec<&str>) {
    // find min residual capacity along the augmenting path
    let min_c = bfs_path
        .windows(2)
        .map(|neighbors| {
            let start = neighbors[0];
            let end = neighbors[1];
            components
                .get(&start)
                .expect("failed to find start node adjacency list")
                .iter()
                .find(|node| node.id == end)
                .expect("failed to find end node")
                .residual_capacity()
        })
        .min()
        .expect("failed to find min residual capacity");

    // update the residual graph
    bfs_path.windows(2).for_each(|neighbors| {
        let start = neighbors[0];
        let end = neighbors[1];

        // update forward edge
        if let Some(nodes) = components.get_mut(start) {
            let node = nodes
                .iter_mut()
                .find(|node| node.id == end)
                .expect("failed to find edge");
            node.flow += min_c;
        }

        // update backward edge
        if let Some(nodes) = components.get_mut(end) {
            let node = nodes
                .iter_mut()
                .find(|node| node.id == start)
                .expect("failed to find edge");
            node.flow -= min_c;
        }
    });
}

fn bfs<'a>(
    components: &HashMap<&str, Vec<Node<'a>>>,
    source: &'a str,
    sink: &'a str,
) -> Option<Vec<&'a str>> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut heap: VecDeque<(&str, Vec<&str>)> = VecDeque::from([(source, Vec::new())]);
    let mut bfs_path: Vec<&str> = Vec::new();
    while let Some((id, mut node_path)) = heap.pop_front() {
        if !visited.insert(id) {
            continue;
        }

        node_path.push(id);
        bfs_path = node_path.clone();
        if id == sink {
            break;
        }

        let neighbors = components
            .get(&id)
            .unwrap_or_else(|| panic!("failed to find node {id}"));
        for node in neighbors {
            if node.residual_capacity() > 0 {
                heap.push_back((node.id, node_path.clone()));
            }
        }
    }

    if *bfs_path.last().unwrap() != sink {
        return None;
    }

    Some(bfs_path)
}

fn parse_input(input: &str) -> HashMap<&str, Vec<Node>> {
    let mut components = HashMap::new();
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .for_each(|l| {
            if let Some((component_name, neighbors)) = l.split_once(": ") {
                let neighbors_set: Vec<Node> = neighbors
                    .split(' ')
                    .map(|neighbor| Node {
                        id: neighbor,
                        capacity: 1,
                        flow: 0,
                    })
                    .collect();
                for neighbor in neighbors_set.iter() {
                    let e: &mut Vec<Node> = components.entry(neighbor.id).or_default();
                    e.push(Node {
                        id: component_name,
                        capacity: 1,
                        flow: 0,
                    });
                }
                let e: &mut Vec<Node> = components.entry(component_name).or_default();
                e.extend(neighbors_set);
            }
        });

    components
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<'a> {
    id: &'a str,
    capacity: isize,
    flow: isize,
}

impl<'a> Node<'a> {
    fn residual_capacity(&'a self) -> isize {
        self.capacity - self.flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        "#;
        assert_eq!(54, process(input, 3));
    }
}
