use std::collections::HashMap;
use crate::core::probability::bayes::BN_base::{State, CPTBase};

pub struct BinaryCPT {
    pub table: Vec<(Vec<State>, f64)>, // padres + P(valor=true)
}

impl CPTBase for BinaryCPT {
    fn get_probability(&self, parent_values: &[State], value: State) -> Option<f64> {
        self.table
            .iter()
            .find(|(parents, _)| parents == parent_values)
            .map(|(_, p)| match value {
                State::True => *p,
                State::False => 1.0 - *p,
                State::Value(_) => todo!(),
            })
    }

    fn possible_values(&self) -> Vec<State> {
        vec![State::True, State::False]
    }

    fn parent_combinations(&self) -> Vec<Vec<State>> {
        self.table.iter().map(|(parents, _)| parents.clone()).collect()
    }
}


pub struct DiscreteCPT {
    // 1. Almacenar los posibles valores del nodo directamente (más eficiente)
    pub node_possible_values: Vec<State>,
    pub table: HashMap<Vec<State>, HashMap<State, f64>>,
}

impl CPTBase for DiscreteCPT {
    fn get_probability(&self, parent_values: &[State], value: State) -> Option<f64> {
        self.table
            .get(parent_values)
            .and_then(|distribution| distribution.get(&value).copied())
    }

    fn possible_values(&self) -> Vec<State> {
        self.node_possible_values.clone()
    }

    fn parent_combinations(&self) -> Vec<Vec<State>> {
        self.table.keys().cloned().collect()
    }
}