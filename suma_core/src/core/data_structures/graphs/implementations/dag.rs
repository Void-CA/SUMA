
use crate::core::data_structures::graphs::traits::graph_base::GraphBase;
use std::hash::Hash;
use super::{DirectedGraph};


pub struct DAG<T> {
    graph : DirectedGraph<T>,
}

impl<T: Eq + Hash + Clone> DAG<T> {
    pub fn new() -> Self {
        Self {
            graph: DirectedGraph::new(),
        }
    }

    pub fn add_node(&mut self, value: T) -> usize {
        self.graph.base.add_node(value)
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        self.graph.add_directed_edge(from, to);

        if self.graph.has_cycle() {
            self.graph.base.edges.remove(&(from, to));
            self.graph.adjacency.get_mut(&from).map(|v| v.retain(|x| *x != to));
            return Err("Adding this edge creates a cycle, cannot add to DAG.");
        }

        Ok(())
    }

    pub fn nodes(&self) -> Vec<usize> {
        self.graph.nodes()
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    pub fn node_data(&self, id: usize) -> Option<&T> {
        self.graph.node_data(id)
    }

    pub fn edge_data(&self, from: usize, to: usize) -> Option<&()> {
        self.graph.edge_data(from, to)
    }
    

}
