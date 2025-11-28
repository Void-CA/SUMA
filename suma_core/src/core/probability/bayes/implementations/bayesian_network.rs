use std::hash::Hash;
use std::collections::HashMap;
use crate::core::data_structures::dag::DAG;
use crate::core::data_structures::graphs::{Directed, GraphBase};
use crate::core::probability::bayes::models::BN_base::*;

pub struct BayesianNetwork<T> {
    dag: DAG<T>,
    cpts: HashMap<usize, Box<dyn CPTBase>>,
}

pub struct Node {
    id: usize,
    parents: Vec<usize>,
    cpt: Box<dyn CPTBase>
}


impl<T> BayesianNetworkBase for BayesianNetwork<T> {
    fn get_nodes(&self) -> Vec<usize> {
        self.dag.nodes()
    }

    fn get_edges(&self) -> Vec<(usize, usize)> {
        self.dag.edges()
    }

    fn get_parents(&self, node: usize) -> Vec<usize> {
        self.dag.predecessors(node)
    }

    fn get_children(&self, node: usize) -> Vec<usize> {
        self.dag.successors(node)
    }

    fn get_cpt(&self, node: usize) -> Option<&(dyn CPTBase + 'static)> {
        // Desreferencia el Box para obtener una referencia al Trait Object
        self.cpts.get(&node).map(|b| b.as_ref())
    }

    fn get_mut_cpt(&mut self, node: usize) -> Option<&mut (dyn CPTBase + 'static)> {
        self.cpts.get_mut(&node).map(|b| b.as_mut())
    }

    fn remove_node(&mut self, node: usize) -> Option<()> {
        if self.dag.remove_node(node).is_some() {
            self.cpts.remove(&node);
            Some(())
        } else {
            None
        }
    }
}

impl<T> BayesianNetwork<T> {
    pub fn new(dag: DAG<T>, cpts: HashMap<usize, Box<dyn CPTBase>>) -> Self {
        BayesianNetwork { dag, cpts }
    }

    pub fn topological_order(&self) -> Result<Vec<usize>, &str> {
        crate::core::data_structures::graphs::topological_sort(&self.dag)
    }

    pub fn rejection_sampling(
        &self,
        evidence: &HashMap<usize, State>,
        query: usize,
        n_samples: usize
    ) -> f64 {
        super::super::algorithms::sampling::rejection_sampling(self, evidence, query, n_samples)
    }

    pub fn get_parent_values(
        &self,
        node: &usize,
        sample: &HashMap<usize, State>,
    ) -> Vec<State> {
        let parents = self.get_parents(*node);
        parents
            .iter()
            .map(|p| sample.get(p).cloned().unwrap_or(State::False))
            .collect()
    }

    pub fn sample_node(&self, node: &usize, parent_values: &[State]) -> State {
        if let Some(cpt) = self.get_cpt(*node) {
            let prob_true = cpt.get_probability(parent_values, State::True).unwrap_or(0.0);
            let rand_val: f64 = rand::random();
            if rand_val < prob_true {
                State::True
            } else {
                State::False
            }
        } else {
            State::False // Valor por defecto si no hay CPT
        }
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