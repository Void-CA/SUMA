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
    ///
    fn get_probability(&self, parent_values: &[&str], value: &str) -> Option<f64>;

    ///
    fn possible_values(&self) -> Vec<String>;

    ///
    fn parent_combinations(&self) -> Vec<Vec<String>>;
}
