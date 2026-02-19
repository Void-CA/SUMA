use std::collections::{HashMap, HashSet};
use num_traits::{Num};
use ordered_float::OrderedFloat;
use crate::data_structures::graphs::{BaseGraph, Directed, GraphBase};
use crate::data_structures::graphs::traits::WeightedGraph;
use crate::data_structures::graphs::weighted::{IntoWeight, Weight};
use crate::formatting::error::ExportError;
use crate::formatting::visualizable::{ToDot, ToMermaid, ToPlantUml};

#[derive(Debug, Clone)]
pub struct UndirectedWeightedGraph<N, E: Weight = OrderedFloat<f64>> {
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

impl<N> UndirectedWeightedGraph<N, OrderedFloat<f64>> {
    pub fn new_float() -> Self {
        Self {
            base: BaseGraph::new(),
            adjacency: HashMap::new(),
        }
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
    pub fn get_id(&self, data: &N) -> Option<usize> 
    where N: PartialEq
    {
        self.base.nodes.iter().find_map(|(id, d)| if *d == *data { Some(*id) } else { None })
    }

    fn add_weighted_edge(&mut self, a: usize, b: usize, weight: E) {
        self.base.nodes.entry(a).or_insert_with(N::default);
        self.base.nodes.entry(b).or_insert_with(N::default);

        self.base.add_edge(a, b, weight);
        self.base.add_edge(b, a, weight);

        self.adjacency.entry(a).or_insert_with(HashSet::new).insert(b);
        self.adjacency.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    pub fn add_node(&mut self, data: N) -> usize {
        self.base.add_node(data)
    }

    pub fn add_edge_id<W>(&mut self, from: usize, to: usize, weight: W) 
    where W: IntoWeight<E>
    {

        self.add_weighted_edge(from, to, weight.into_weight());
    }

    pub fn add_edge<W>(&mut self, from: N, to: N, weight: W)
    where
        W: IntoWeight<E>,
        N: PartialEq,
    {
        let from_id = self.base.get_or_add_node(from);
        let to_id = self.base.get_or_add_node(to);

        self.add_weighted_edge(from_id, to_id, weight.into_weight());
    }


    pub fn nodes(&self) -> Vec<usize> {
        self.base.nodes.keys().cloned().collect()
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.base.edges
            .keys()
            .filter(|(a, b)| a <= b)
            .cloned()
            .collect()
    }

    pub fn edge_data(&self, from: usize, to: usize) -> Option<&E> {
        self.base.edges.get(&(from, to))
    }

}

impl<N, E: Weight + std::fmt::Display> ToDot for UndirectedWeightedGraph<N, E> {
    fn to_dot(&self) -> Result<String, ExportError> {
        let mut s = String::from("graph G {\n");

        for (from, to) in self.edges() {
            if let Some(weight) = self.edge_data(from, to) {
                s.push_str(&format!("  {} -- {} [label=\"{}\"];\n", from, to, weight));
            }
        }

        s.push('}');
        Ok(s)
    }
}


impl<N, E: Weight + std::fmt::Display> ToMermaid for UndirectedWeightedGraph<N, E> {
    fn to_mermaid(&self) -> Result<String, ExportError> {
        let mut s = String::from("graph TD\n");

        for (from, to) in self.edges() {
            if let Some(weight) = self.edge_data(from, to) {
                s.push_str(&format!("  {} ---|{}| {}\n", from, weight, to));
            }
        }

        Ok(s)
    }
}

impl<N, E: Weight + std::fmt::Display> ToPlantUml for UndirectedWeightedGraph<N, E> {
    fn to_plantuml(&self) -> Result<String, ExportError> {
        let mut s = String::from("@startuml\n");

        for (from, to) in self.edges() {
            if let Some(weight) = self.edge_data(from, to) {
                s.push_str(&format!("  {} -- {} : {}\n", from, to, weight));
            }
        }

        s.push_str("@enduml");
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use num_traits::float;
    use ordered_float::OrderedFloat;
    use super::*;
    use crate::{float_uwgraph, uwgraph};
    
    #[test]
    fn test_ergonomics() {
        let mut g = float_uwgraph! {
            A => {
                B: 1.5,
                C: 2.0
            },
            B => {
                A: 1.5,
                C: 3.0
            },
            C => {
                A: 2.0,
                B: 3.0
            }
        };
    }

    #[test]
    fn test_integer_weights() {
        let mut graph = UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node("node1");
        let node2 = graph.base.add_node("node2");

        graph.add_edge_id(node1, node2, 5); // Usando i32

        assert_eq!(graph.edge_weight(node1, node2), Some(5));
        assert_eq!(graph.edge_data(node1, node2), Some(&5));
    }

    #[test]
    fn test_float_weights() {
        let mut graph= UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node("node1");
        let node2 = graph.base.add_node("node2");

        graph.add_edge_id(node1, node2, OrderedFloat::from(3.5));

        assert_eq!(graph.edge_weight(node1, node2), Some(3.5.into()));
    }

    #[test]
    fn test_path_weight() {
        let mut graph = UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);
        let node3 = graph.base.add_node(3);

        graph.add_edge_id(node1, node2, 2);
        graph.add_edge_id(node2, node3, 3);

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
        graph_i32.add_edge_id(n1, n2, 100);
        assert_eq!(graph_i32.edge_weight(n1, n2), Some(100));

        // Test con f32
        let mut graph_f32: UndirectedWeightedGraph<&str, OrderedFloat<f64>> = UndirectedWeightedGraph::new();
        let n1 = graph_f32.base.add_node("A");
        let n2 = graph_f32.base.add_node("B");
        graph_f32.add_edge_id(n1, n2, 1.5);
        assert_eq!(graph_f32.edge_weight(n1, n2), Some(1.5.into()));
    }

    #[test]
    fn test_visualization() {
        let mut graph = UndirectedWeightedGraph::new();
        let node1 = graph.base.add_node("node1");
        let node2 = graph.base.add_node("node2");

        graph.add_edge_id(node1, node2, 4);

        let dot_representation = graph.to_dot().unwrap();
    }
}