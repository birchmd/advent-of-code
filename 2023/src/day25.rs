use {
    aoc_core::{
        graph::{Node, UndirectedWeightedGraph},
        Solution,
    },
    std::collections::{HashMap, HashSet},
};

pub struct Day25;

impl<'a> Solution<'a> for Day25 {
    type Input = HashMap<&'a str, Vec<&'a str>>;
    type Output1 = usize;
    type Output2 = ();

    fn parse_input(data: &'a str) -> Self::Input {
        let mut result: HashMap<&str, Vec<&str>> = HashMap::new();

        for line in data.lines() {
            let (a, remainder) = line.split_once(':').expect("Has colon");
            for b in remainder.trim().split(' ') {
                result.entry(a).or_default().push(b);
                result.entry(b).or_default().push(a);
            }
        }

        result
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let n = input.len();
        let m = stoer_wagner_algorithm(input);
        m * (n - m)
    }

    fn part_2(_input: Self::Input) -> Self::Output2 {
        // It's day 25 so there is no part 2 :)
    }
}

// See https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
fn stoer_wagner_algorithm<'a>(graph: HashMap<&'a str, Vec<&'a str>>) -> usize {
    let mut shrinking_graph = UndirectedWeightedGraph::unweighted(graph);

    let start = shrinking_graph.inner.keys().next().cloned().unwrap();

    let mut min_cut = Cut {
        weight: u64::MAX,
        severed: Node { name: Vec::new() },
    };
    while shrinking_graph.inner.len() > 1 {
        let cut_of_phase = stoer_wagner_phase(&start, &mut shrinking_graph);
        if cut_of_phase.weight < min_cut.weight {
            min_cut = cut_of_phase;
            if min_cut.weight == 3 {
                break;
            }
        }
    }

    min_cut.severed.name.len()
}

fn stoer_wagner_phase<'a>(
    start: &Node<&'a str>,
    graph: &mut UndirectedWeightedGraph<&'a str>,
) -> Cut<'a> {
    let mut set = HashSet::new();
    let mut boundary_nodes = HashMap::new();

    set.insert(start);
    for (node, weight) in &graph.inner[start] {
        boundary_nodes.insert(node, *weight);
    }

    let mut second_last = start;
    let mut last = start;
    while set.len() < graph.inner.len() {
        let next_to_add = boundary_nodes
            .keys()
            .max_by_key(|z| boundary_nodes[**z])
            .copied()
            .unwrap();
        boundary_nodes.remove(&next_to_add);
        set.insert(next_to_add);
        for (node, weight) in &mut boundary_nodes {
            *weight += graph.weight(next_to_add, *node);
        }
        let total_weight = |z: &Node<&'a str>| -> u64 {
            graph.inner[z]
                .iter()
                .filter_map(|(a, weight)| if set.contains(a) { Some(*weight) } else { None })
                .sum()
        };
        for node in graph.neighbors_of(next_to_add) {
            if !set.contains(node) {
                boundary_nodes.insert(node, total_weight(node));
            }
        }
        second_last = last;
        last = next_to_add;
    }
    // The cut of the phase is the cut which separates the last added node from
    // the rest of the graph.
    let weight = graph.inner[last].values().copied().sum();
    let cut = Cut {
        weight,
        severed: last.clone(),
    };

    graph.merge_nodes(&second_last.clone(), &last.clone());
    cut
}

struct Cut<'a> {
    weight: u64,
    severed: Node<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day25_example.txt");

    #[test]
    fn test_part1() {
        let input = Day25::parse_input(EXAMPLE_INPUT);
        let output = Day25::part_1(input);
        assert_eq!(output, 54);
    }
}
