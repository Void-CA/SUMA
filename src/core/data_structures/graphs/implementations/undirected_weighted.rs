use std::collections::{HashMap, HashSet};
use num_traits::{Num};
use crate::core::data_structures::graphs::{BaseGraph, GraphBase};
use crate::core::data_structures::graphs::traits::WeightedGraph;
use crate::core::data_structures::graphs::weighted::Weight;

#[derive(Debug, Clone)]
pub struct UndirectedWeightedGraph<N, E: Weight> {
    pub base: BaseGraph<N, E>,
    pub adjacency: HashMap<usize, HashSet<usize>>,
}

impl<N, E: Weight> UndirectedWeightedGraph<N, E> {
    pub fn new() -> Self {
        Self {
            base: BaseGraph::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn path_weight(&self, path: &[usize]) -> Option<E> {
        if path.len() < 2 {
            return Some(E::zero());
        }

        let mut total = E::zero();
        for window in path.windows(2) {
            if let Some(&weight) = self.edge_data(window[0], window[1]) {
                total = total + weight;
            } else {
                return None;
            }
        }
        Some(total)
    }
}

impl<N, E: Weight> GraphBase for UndirectedWeightedGraph<N, E> {
    type NodeId = usize;
    type NodeData = N;
    type EdgeData = E;

    fn nodes(&self) -> Vec<usize> {
        self.base.nodes.keys().cloned().collect()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.base.edges
            .keys()
            .filter(|(a, b)| a <= b)
            .cloned()
            .collect()
    }

    fn node_data(&self, id: usize) -> Option<&N> {
        self.base.nodes.get(&id)
    }

    fn edge_data(&self, from: usize, to: usize) -> Option<&E> {
        self.base.edges.get(&(from, to))
    }

    fn neighbors(&self, node: Self::NodeId) -> Vec<Self::NodeId> {
        self.adjacency
            .get(&node)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }
}

impl<N, E: Weight> WeightedGraph for UndirectedWeightedGraph<N, E> {
    fn edge_weight(&self, from: usize, to: usize) -> Option<E> {
        self.edge_data(from, to).copied()
    }
}


impl<N: Default, E: Weight> UndirectedWeightedGraph<N, E> {
    pub fn add_weighted_edge(&mut self, a: usize, b: usize, weight: E) {
        self.base.nodes.entry(a).or_insert_with(N::default);
        self.base.nodes.entry(b).or_insert_with(N::default);

        self.base.add_edge(a, b, weight);
        self.base.add_edge(b, a, weight);

        self.adjacency.entry(a).or_insert_with(HashSet::new).insert(b);
        self.adjacency.entry(b).or_insert_with(HashSet::new).insert(a);
    }
}


#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;
    use super::*;

    #[test]
    fn test_integer_weights() {
        let mut graph = UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node("node1");
        let node2 = graph.base.add_node("node2");

        graph.add_weighted_edge(node1, node2, 5); // Usando i32

        assert_eq!(graph.edge_weight(node1, node2), Some(5));
        assert_eq!(graph.edge_data(node1, node2), Some(&5));
    }

    #[test]
    fn test_float_weights() {
        let mut graph= UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node("node1");
        let node2 = graph.base.add_node("node2");

        graph.add_weighted_edge(node1, node2, OrderedFloat::from(3.5));

        assert_eq!(graph.edge_weight(node1, node2), Some(3.5.into()));
    }

    #[test]
    fn test_path_weight() {
        let mut graph = UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);
        let node3 = graph.base.add_node(3);

        graph.add_weighted_edge(node1, node2, 2);
        graph.add_weighted_edge(node2, node3, 3);

        let path = vec![node1, node2, node3];
        assert_eq!(graph.path_weight(&path), Some(5));

        let invalid_path = vec![node1, node3];
        assert_eq!(graph.path_weight(&invalid_path), None);
    }
    

    #[test]
    fn test_different_numeric_types() {
        // Test con i32
        let mut graph_i32: UndirectedWeightedGraph<&str, i32> = UndirectedWeightedGraph::new();
        let n1 = graph_i32.base.add_node("A");
        let n2 = graph_i32.base.add_node("B");
        graph_i32.add_weighted_edge(n1, n2, 100);
        assert_eq!(graph_i32.edge_weight(n1, n2), Some(100));

        // Test con f32
        let mut graph_f32: UndirectedWeightedGraph<&str, OrderedFloat<f64>> = UndirectedWeightedGraph::new();
        let n1 = graph_f32.base.add_node("A");
        let n2 = graph_f32.base.add_node("B");
        graph_f32.add_weighted_edge(n1, n2, 1.5.into());
        assert_eq!(graph_f32.edge_weight(n1, n2), Some(1.5.into()));
    }

}