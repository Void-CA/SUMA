use std::collections::HashMap;
use crate::core::data_structures::DirectedGraph;
use crate::core::data_structures::graphs::GraphBase;
use crate::core::probability::bayes::models::BN_base::*;

pub struct BayesianNetwork<T> {
    graph: DirectedGraph<T>,
    cpts: HashMap<usize, Box<dyn CPTBase>>,
}

pub struct Node {
    id: usize,
    parents: Vec<usize>,
    cpt: Box<dyn CPTBase>
}


impl<T> BayesianNetworkBase for BayesianNetwork<T> {
    fn get_nodes(&self) -> Vec<usize> {
        self.graph.nodes()
    }

    fn get_edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    fn get_parents(&self, node: usize) -> Vec<usize> {
        self.get_parents(node)
    }

    fn get_children(&self, node: usize) -> Vec<usize> {
        self.get_children(node)
    }

    fn get_cpt(&self, node: usize) -> Option<&(dyn CPTBase + 'static)> {
        // Desreferencia el Box para obtener una referencia al Trait Object
        self.cpts.get(&node).map(|b| b.as_ref())
    }

    fn get_mut_cpt(&mut self, node: usize) -> Option<&mut (dyn CPTBase + 'static)> {
        self.cpts.get_mut(&node).map(|b| b.as_mut())
    }

    fn remove_node(&mut self, node: usize) -> Option<()> {
        if self.graph.remove_node(node).is_some() {
            self.cpts.remove(&node);
            Some(())
        } else {
            None
        }
    }
}

impl<T> BayesianNetwork<T> {
    pub fn new(graph: DirectedGraph<T>, cpts: HashMap<usize, Box<dyn CPTBase>>) -> Self {
        BayesianNetwork { graph, cpts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayesian_network() {
        // Aquí irían las pruebas para la implementación de la red bayesiana
    }
}