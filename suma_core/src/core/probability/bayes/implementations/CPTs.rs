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

    fn new_no_parents(_possible_values: Vec<State>, probabilities: Vec<f64>) -> Self where Self: Sized {
        assert_eq!(probabilities.len(), 1, "BinaryCPT with no parents should have exactly one probability.");
        Self {
            table: vec![(vec![], probabilities[0])],
        }
    }
    fn new_with_parents(parent_combinations: Vec<Vec<State>>, probabilities: Vec<HashMap<State, f64>>, _possible_values: Vec<State>) -> Self
    where
        Self: Sized,
    {
        let mut table = Vec::new();
        for (parents, distribution) in parent_combinations.into_iter().zip(probabilities.into_iter()) {
            let p_true = *distribution.get(&State::True).expect("Missing probability for State::True");
            table.push((parents, p_true));
        }
        Self {
            table,
        }
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

    fn new_no_parents(possible_values: Vec<State>, probabilities: Vec<f64>) -> Self {
        let mut distribution = HashMap::new();
        for (i, &prob) in probabilities.iter().enumerate() {
            distribution.insert(possible_values[i].clone(), prob);
        }

        DiscreteCPT {
            node_possible_values: possible_values,
            table: HashMap::from([(vec![], distribution)]),
        }
    }

    fn new_with_parents(
        parent_combinations: Vec<Vec<State>>,
        probabilities: Vec<HashMap<State, f64>>,
        possible_values: Vec<State>,
    ) -> Self {
        let mut table = HashMap::new();
        for (i, parent_combo) in parent_combinations.into_iter().enumerate() {
            table.insert(parent_combo, probabilities[i].clone());
        }

        DiscreteCPT {
            node_possible_values: possible_values,
            table,
        }
    }
}