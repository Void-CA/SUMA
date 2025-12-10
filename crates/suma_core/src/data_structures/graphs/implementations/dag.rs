
use crate::data_structures::graphs::traits::graph_base::GraphBase;
use std::hash::Hash;
use crate::data_structures::graphs::Directed;
use crate::formatting::error::ExportError;
use crate::formatting::visualizable::{ToDot, ToMermaid, ToPlantUml};
use super::{DirectedGraph};


pub struct DAG<T> {
    graph : DirectedGraph<T>,
}

impl<T> DAG<T> {
    pub fn new() -> Self {
        Self {
            graph: DirectedGraph::new(),
        }
    }

    pub fn add_node(&mut self, value: T) -> usize {
        self.graph.add_node(value)
    }

    pub fn remove_node(&mut self, node: usize) -> Option<T> {
        self.graph.remove_node(node)
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        self.graph.add_directed_edge(from, to);

        if self.graph.has_cycle() {
            self.graph.remove_edge(from, to);
            return Err("Adding this edge creates a cycle, cannot add to DAG.");
        }

        Ok(())
    }

}

impl<T> GraphBase for DAG<T> {
    type NodeId = usize;
    type NodeData = T;
    type EdgeData = ();

    fn nodes(&self) -> Vec<Self::NodeId> {
        self.graph.nodes()
    }

    fn edges(&self) -> Vec<(Self::NodeId, Self::NodeId)> {
        self.graph.edges()
    }

    fn node_data(&self, id: Self::NodeId) -> Option<&Self::NodeData> {
        self.graph.node_data(id)
    }

    fn edge_data(&self, from: Self::NodeId, to: Self::NodeId) -> Option<&Self::EdgeData> {
        self.graph.edge_data(from, to)
    }

    fn neighbors(&self, node: Self::NodeId) -> Vec<Self::NodeId> {
        self.graph.neighbors(node)
    }
}



impl<T> Directed for DAG<T> {
    fn predecessors(&self, node: Self::NodeId) -> Vec<Self::NodeId> {
        self.graph.predecessors(node)
    }
    fn successors(&self, node: Self::NodeId) -> Vec<Self::NodeId> {
        self.graph.successors(node)
    }
}

impl<T> ToMermaid for DAG<T> {
    fn to_mermaid(&self) -> Result<String, ExportError> {
        self.graph.to_mermaid()
    }
}


impl<T> ToPlantUml for DAG<T> {
    fn to_plantuml(&self) -> Result<String, ExportError> {
        self.graph.to_plantuml()
    }
}

impl<T> ToDot for DAG<T> {
    fn to_dot(&self) -> Result<String, ExportError> {
        self.graph.to_dot()
    }
}

impl<T> DAG<T> where T: Hash + Eq {
    pub fn has_cycle(&self) -> bool {
        self.graph.has_cycle()
    }
}