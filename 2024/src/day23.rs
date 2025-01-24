use {
    aoc_core::Solution,
    std::collections::{HashMap, HashSet},
};

pub struct Day23;

impl<'a> Solution<'a> for Day23 {
    type Input = Graph<'a>;
    type Output1 = usize;
    type Output2 = String;

    fn parse_input(data: &'a str) -> Self::Input {
        let mut result: HashMap<&str, HashSet<&str>> = HashMap::new();
        for line in data.lines() {
            let (a, b) = line.split_once('-').expect("Contains dash");
            result.entry(a).or_default().insert(b);
            result.entry(b).or_default().insert(a);
        }
        result
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        find_triples(&input)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        find_password(&input)
    }
}

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn find_triples(graph: &Graph<'_>) -> usize {
    let mut triples = HashSet::new();
    for (node, neighbors) in graph {
        if !node.starts_with('t') || neighbors.len() < 2 {
            continue;
        }

        for a in neighbors {
            for b in neighbors {
                if a == b {
                    continue;
                }
                if graph[a].contains(b) {
                    let mut triple = [a, b, node];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        }
    }
    triples.len()
}

// See https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    graph: &Graph<'a>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    for v in p.clone() {
        let neighbors = &graph[v];
        let mut r_prime = r.clone();
        r_prime.insert(v);
        bron_kerbosch(
            r_prime,
            p.intersection(neighbors).copied().collect(),
            x.intersection(neighbors).copied().collect(),
            graph,
            cliques,
        );
        p.remove(v);
        x.insert(v);
    }
}

fn find_password(graph: &Graph<'_>) -> String {
    let mut cliques = Vec::new();
    bron_kerbosch(
        HashSet::new(),
        graph.keys().copied().collect(),
        HashSet::new(),
        graph,
        &mut cliques,
    );
    let maximum_clique = cliques.iter().max_by_key(|c| c.len()).unwrap();

    let mut alphabetical: Vec<&str> = maximum_clique.iter().copied().collect();
    alphabetical.sort();

    alphabetical.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day23_example.txt");

    #[test]
    fn test_part1() {
        let input = Day23::parse_input(EXAMPLE_INPUT);
        let output = Day23::part_1(input);
        assert_eq!(output, 7);
    }

    #[test]
    fn test_part2() {
        let input = Day23::parse_input(EXAMPLE_INPUT);
        let output = Day23::part_2(input);
        assert_eq!(output, "co,de,ka,ta");
    }
}
