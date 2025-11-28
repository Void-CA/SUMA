use std::fmt::Debug;


pub trait BayesianNetworkBase {
    fn get_nodes(&self) -> Vec<usize>;
    fn get_edges(&self) -> Vec<(usize, usize)>;
    fn get_parents(&self, node: usize) -> Vec<usize>;
    fn get_children(&self, node: usize) -> Vec<usize>;

    /// Returns the Conditional Probability Table (CPT) for a given node.
    fn get_cpt(&self, node: usize) -> Option<&(dyn CPTBase + 'static)>;

    fn get_mut_cpt(&mut self, node: usize) -> Option<&mut (dyn CPTBase + 'static)>;

    fn remove_node(&mut self, node: usize) -> Option<()>;
}

pub trait CPTBase {
    fn get_probability(&self, parent_values: &[State], value: State) -> Option<f64>;
    fn possible_values(&self) -> Vec<State>;
    fn parent_combinations(&self) -> Vec<Vec<State>>;
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    True,
    False,
    Value(String), // para otros casos categóricos
}

impl State {
    pub fn from_str(s: &str) -> State {
        match s.to_lowercase().as_str() {
            "true" => State::True,
            "false" => State::False,
            _ => State::Value(s.to_string()),
        }
    }
}