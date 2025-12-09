use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::probability::bayes::BN_base::{State, CPTBase};
use crate::core::probability::utils::serialize::serialize_complex_key;

#[derive(Serialize, Deserialize)]
pub struct BinaryCPT {
    pub table: Vec<(Vec<State>, f64)>, // padres + P(valor=true)
}

impl CPTBase for BinaryCPT {
    fn get_probability(&self, parent_values: &[State], value: State) -> Option<f64> {
        self.table
            .iter()
            .find(|(parents, _)| parents == parent_values)
            .map(|(_, p)| match value {
                State::True => Some(*p),
                State::False => Some(1.0 - *p),
                State::Value(_) => return None,
            })
            .flatten()
    }

    fn possible_values(&self) -> Vec<State> {
        vec![State::True, State::False]
    }

    fn parent_combinations(&self) -> Vec<Vec<State>> {
        self.table.iter().map(|(parents, _)| parents.clone()).collect()
    }

    fn sample(&self, parent_values: &[State]) -> Option<State> {
        let p_true = self.get_probability(parent_values, State::True)?;
        let rand: f64 = crate::core::probability::utils::random::random_f64();
        if rand <= p_true {
            Some(State::True)
        } else {
            Some(State::False)
        }
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


#[derive(Serialize, Deserialize)]
pub struct DiscreteCPT {
    // 1. Almacenar los posibles valores del nodo directamente (más eficiente)
    pub node_possible_values: Vec<State>,

    #[serde(serialize_with = "serialize_complex_key")]
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

    fn sample(&self, parent_values: &[State]) -> Option<State> {
        let distribution = self.table.get(parent_values)?;
        let mut cumulative_prob = 0.0;
        let rand: f64 = crate::core::probability::utils::random::random_f64();

        for state in &self.node_possible_values {
            if let Some(&prob) = distribution.get(state) {
                cumulative_prob += prob;
                if rand <= cumulative_prob {
                    return Some(state.clone());
                }
            }
        }
        None
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

#[derive(Serialize, Deserialize)]
pub enum CPT {
    Binary(BinaryCPT),
    Discrete(DiscreteCPT),
}

impl CPTBase for CPT {
    fn get_probability(&self, parent_values: &[State], value: State) -> Option<f64> {
        match self {
            CPT::Binary(binary) => binary.get_probability(parent_values, value),
            CPT::Discrete(discrete) => discrete.get_probability(parent_values, value),
        }
    }

    fn possible_values(&self) -> Vec<State> {
        match self {
            CPT::Binary(binary) => binary.possible_values(),
            CPT::Discrete(discrete) => discrete.possible_values(),
        }
    }

    fn parent_combinations(&self) -> Vec<Vec<State>> {
        match self {
            CPT::Binary(binary) => binary.parent_combinations(),
            CPT::Discrete(discrete) => discrete.parent_combinations(),
        }
    }

    fn sample(&self, parent_values: &[State]) -> Option<State> {
        match self {
            CPT::Binary(binary) => binary.sample(parent_values),
            CPT::Discrete(discrete) => discrete.sample(parent_values),
        }
    }

    fn new_no_parents(possible_values: Vec<State>, probabilities: Vec<f64>) -> Self {
        if possible_values.len() == 2 && possible_values.contains(&State::True) && possible_values.contains(&State::False) {
            CPT::Binary(BinaryCPT::new_no_parents(possible_values, probabilities))
        } else {
            CPT::Discrete(DiscreteCPT::new_no_parents(possible_values, probabilities))
        }
    }

    fn new_with_parents(
        parent_combinations: Vec<Vec<State>>,
        probabilities: Vec<HashMap<State, f64>>,
        possible_values: Vec<State>,
    ) -> Self {
        if possible_values.len() == 2 && possible_values.contains(&State::True) && possible_values.contains(&State::False) {
            CPT::Binary(BinaryCPT::new_with_parents(parent_combinations, probabilities, possible_values))
        } else {
            CPT::Discrete(DiscreteCPT::new_with_parents(parent_combinations, probabilities, possible_values))
        }
    }
}