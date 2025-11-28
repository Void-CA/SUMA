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
