use std::collections::HashMap;
use super::super::traits::GraphBase;
use crate::formatting::visualizable::{ToDot, ToMermaid, ToPlantUml};

#[derive(Debug, Clone)]
pub struct BaseGraph<T, E> {
    pub nodes: HashMap<usize, T>,
    pub edges: HashMap<(usize, usize), E>,
    pub next_id: usize,
}

impl<T, E> BaseGraph<T, E> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_node(&mut self, data: T) -> usize {
        let id = self.next_id;
        self.nodes.insert(id, data);
        self.next_id += 1;
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge_data: E) {
        self.edges.insert((from, to), edge_data);
    }

    pub fn remove_node(&mut self, id: usize) -> Option<T> {
        self.edges.retain(|(from, to), _| *from != id && *to != id);
        self.nodes.remove(&id)
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> Option<E> {
        self.edges.remove(&(from, to))
    }
}

use std::hash::Hash;
use crate::formatting::error::ExportError;

impl<T, E> GraphBase for BaseGraph<T, E>
where
    T: Clone,
{
    type EdgeData = E;

    fn nodes(&self) -> Vec<Self::NodeId> {
        self.nodes.keys().cloned().collect()
    }

    fn edges(&self) -> Vec<(Self::NodeId, Self::NodeId)> {
        self.edges.keys().cloned().collect()
    }

    fn node_data(&self, id: Self::NodeId) -> Option<&Self::NodeData> {
        self.nodes.get(&id)
    }

    fn edge_data(&self, from: Self::NodeId, to: Self::NodeId) -> Option<&Self::EdgeData> {
        self.edges.get(&(from, to))
    }

    fn neighbors(&self, node: Self::NodeId) -> Vec<Self::NodeId> {
        self.edges
            .keys()
            .filter_map(|(from, to)| if *from == node { Some(*to) } else { None })
            .collect()
    }

    type NodeId = usize;
    type NodeData = T;

}

impl<T, E> ToDot for BaseGraph<T, E> {
    fn to_dot(&self) -> Result<String, ExportError> {
        let mut s = String::from("digraph G {\n");

        for ((from, to), _) in &self.edges {
            s.push_str(&format!("  {} -> {};\n", from, to));
        }

        s.push('}');
        Ok(s)
    }
}

impl<T, E> ToMermaid for BaseGraph<T, E> {
    fn to_mermaid(&self) -> Result<String, ExportError> {
        let mut s = String::from("graph TD\n");

        for ((from, to), _) in &self.edges {
            s.push_str(&format!("  {} --> {}\n", from, to));
        }

        Ok(s)
    }
}

impl<T, E> ToPlantUml for BaseGraph<T, E> {
    fn to_plantuml(&self) -> Result<String, ExportError> {
        let mut s = String::from("@startuml\n");

        for ((from, to), _) in &self.edges {
            s.push_str(&format!("  {} --> {}\n", from, to));
        }

        s.push_str("@enduml");
        Ok(s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph: BaseGraph<i32, f64> = BaseGraph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert_eq!(graph.next_id, 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph: BaseGraph<&str, i32> = BaseGraph::new();
        let id1 = graph.add_node("data1");
        let id2 = graph.add_node("data2");

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.next_id, 2);
        assert_eq!(graph.nodes.get(&0), Some(&"data1"));
        assert_eq!(graph.nodes.get(&1), Some(&"data2"));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = BaseGraph::new();
        let n1 = graph.add_node(10);
        let n2 = graph.add_node(20);

        graph.add_edge(n1, n2, 1.5);

        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges.get(&(n1, n2)), Some(&1.5));
    }

    #[test]
    fn test_remove_node() {
        let mut graph: BaseGraph<&str, i32> = BaseGraph::new();
        let id = graph.add_node("test");

        let removed = graph.remove_node(id);

        assert_eq!(removed, Some("test"));
        assert!(!graph.nodes.contains_key(&id));
    }

    #[test]
    fn test_remove_nonexistent_node() {
        let mut graph: BaseGraph<&str, f64> = BaseGraph::new();
        assert_eq!(graph.remove_node(999), None);
    }

    #[test]
    fn test_remove_edge() {
        let mut graph: BaseGraph<i32, &str> = BaseGraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, "edge");

        let removed = graph.remove_edge(n1, n2);

        assert_eq!(removed, Some("edge"));
        assert!(!graph.edges.contains_key(&(n1, n2)));
    }

    #[test]
    fn test_remove_nonexistent_edge() {
        let mut graph: BaseGraph<i32, &str> = BaseGraph::new();
        assert_eq!(graph.remove_edge(0, 1), None);
    }

    #[test]
    fn test_multiple_edges() {
        let mut graph: BaseGraph<i32, &str> = BaseGraph::new();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        graph.add_edge(n0, n1, "edge1");
        graph.add_edge(n1, n2, "edge2");
        graph.add_edge(n0, n2, "edge3");

        assert_eq!(graph.edges.len(), 3);
        assert_eq!(graph.edges.get(&(n0, n1)), Some(&"edge1"));
        assert_eq!(graph.edges.get(&(n1, n2)), Some(&"edge2"));
        assert_eq!(graph.edges.get(&(n0, n2)), Some(&"edge3"));
    }

    #[test]
    fn test_edges_after_node_removal() {
        let mut graph: BaseGraph<i32, &str> = BaseGraph::new();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        graph.add_edge(n0, n1, "edge");

        graph.remove_node(n0);

        // Ahora debería verificar que la arista NO existe
        assert!(!graph.edges.contains_key(&(n0, n1)));
    }

    #[test]
    fn test_dot() {
        let mut graph: BaseGraph<i32, &str> = BaseGraph::new();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        graph.add_edge(n0, n1, "edge");

        let dot_output = graph.to_dot().unwrap();
        let expected_output = "digraph G {\n  0 -> 1;\n}";

        assert_eq!(dot_output, expected_output);
    }
}