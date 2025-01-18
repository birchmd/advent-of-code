use std::{collections::HashMap, hash::Hash};

pub struct UndirectedWeightedGraph<L> {
    pub inner: HashMap<Node<L>, HashMap<Node<L>, u64>>,
}

impl<L: Copy + PartialEq + Eq + Hash> UndirectedWeightedGraph<L> {
    pub fn unweighted<I>(nodes: I) -> Self
    where
        I: IntoIterator<Item = (L, Vec<L>)>,
    {
        let mut inner: HashMap<Node<L>, HashMap<Node<L>, u64>> = HashMap::new();
        for (x, neighbors) in nodes {
            let a = Node::new(x);
            for y in neighbors {
                let b = Node::new(y);
                inner.entry(a.clone()).or_default().insert(b.clone(), 1);
                inner.entry(b).or_default().insert(a.clone(), 1);
            }
        }
        Self { inner }
    }

    pub fn neighbors_of(&self, a: &Node<L>) -> impl Iterator<Item = &Node<L>> {
        self.inner[a].keys()
    }

    pub fn weight(&self, a: &Node<L>, b: &Node<L>) -> u64 {
        self.inner[a].get(b).copied().unwrap_or_default()
    }

    pub fn merge_nodes(&mut self, a: &Node<L>, b: &Node<L>) {
        let new_node = Node::combine(a, b);

        let mut a_neighbors = self.inner.remove(a).unwrap();
        let mut b_neighbors = self.inner.remove(b).unwrap();
        a_neighbors.remove(b);
        b_neighbors.remove(a);

        let mut combined_neighbors = a_neighbors;
        for (n, weight) in b_neighbors {
            *combined_neighbors.entry(n).or_default() += weight;
        }
        for (node, weight) in combined_neighbors.iter() {
            let neighbors = self.inner.get_mut(node).unwrap();
            neighbors.remove(a);
            neighbors.remove(b);
            neighbors.insert(new_node.clone(), *weight);
        }
        self.inner.insert(new_node, combined_neighbors);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<L> {
    pub name: Vec<L>,
}

impl<L> Node<L> {
    pub fn new(name: L) -> Self {
        Self { name: vec![name] }
    }
}

impl<L: Copy> Node<L> {
    pub fn combine(a: &Self, b: &Self) -> Self {
        let default = a.name.first().copied().unwrap();
        let n = a.name.len();
        let m = b.name.len();
        let mut name = vec![default; n + m];
        name[0..n].copy_from_slice(&a.name);
        name[n..(n + m)].copy_from_slice(&b.name);
        Self { name }
    }
}
