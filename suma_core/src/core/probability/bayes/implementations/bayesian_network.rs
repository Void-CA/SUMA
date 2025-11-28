use std::hash::Hash;
use std::collections::HashMap;
use crate::core::data_structures::dag::DAG;
use crate::core::data_structures::graphs::{Directed, GraphBase};
use crate::core::probability::bayes::models::BN_base::*;

pub struct BayesianNetwork {
    dag: DAG<usize>,
    cpts: HashMap<usize, Box<dyn CPTBase>>,
    name_to_id: HashMap<String, usize>,
    id_to_name: HashMap<usize, String>,
}


impl BayesianNetworkBase for BayesianNetwork {
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

impl BayesianNetwork {
    pub fn new() -> Self {
        BayesianNetwork {
            dag: DAG::new(),
            cpts: HashMap::new(),
            name_to_id : HashMap::new(),
            id_to_name : HashMap::new(),
        }
    }

    pub fn from_parts(
        dag: DAG<usize>,
        cpts: HashMap<usize, Box<dyn CPTBase>>,
        name_to_id: HashMap<String, usize>,
    ) -> Self {

        // Invertir name_to_id para generar id_to_name
        let mut id_to_name = HashMap::new();
        for (name, id) in &name_to_id {
            id_to_name.insert(*id, name.clone());
        }

        BayesianNetwork {
            dag,
            cpts,
            name_to_id,
            id_to_name,
        }
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

    fn add_node_dag(&mut self, node: usize, cpt: Box<dyn CPTBase>) {
        self.dag.add_node(node);
        self.cpts.insert(node, cpt);
    }

    pub fn add_edge(&mut self, parent: &str, child: &str) {
        let parent = self.name_to_id.get(parent).expect("Parent node not found.");
        let child = self.name_to_id.get(child).expect("Child node not found.");
        self.dag.add_edge(*parent, *child);
    }

    pub fn get_id_from_name(&self, name: &str) -> Option<usize> {
        self.name_to_id.get(name).copied()
    }

    pub fn get_name_from_id(&self, node_id: usize) -> Option<&String> {
        self.id_to_name.get(&node_id)
    }

    pub fn add_node(
        &mut self,
        name: &str,
        cpt: Box<dyn CPTBase>
    ) -> Result<usize, &'static str> {

        if self.name_to_id.contains_key(name) {
            return Err("Node with this name already exists.");
        }

        // ID incremental seguro: 0,1,2,...
        let node_id = self.cpts.len();

        self.dag.add_node(node_id);
        self.cpts.insert(node_id, cpt);

        self.name_to_id.insert(name.to_string(), node_id);
        self.id_to_name.insert(node_id, name.to_string());

        Ok(node_id)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::probability::bayes::implementations::CPTs::BinaryCPT;
    use crate::core::probability::bayes::BN_base::State;

    // Construye una red bayesiana simple:
    // 0: Rain
    // 1: Sprinkler
    // 2: WetGrass
    fn setup() -> BayesianNetwork {
        let mut bn = BayesianNetwork::new();

        // CPT Rain:   P(Rain=True) = 0.2
        let rain_cpt = BinaryCPT {
            table: vec![(vec![], 0.2)],
        };
        bn.add_node("Rain", Box::new(rain_cpt));

        // CPT Sprinkler: depende de Rain
        // P(S=True | R=True) = 0.01
        // P(S=True | R=False) = 0.40
        let sprinkler_cpt = BinaryCPT {
            table: vec![
                (vec![State::True],  0.01),
                (vec![State::False], 0.40),
            ],
        };
        bn.add_node("Sprinkler", Box::new(sprinkler_cpt));

        // CPT WetGrass: depende de Rain y Sprinkler
        // P(W=True | R=True, S=True) = 0.99
        // P(W=True | R=True, S=False) = 0.80
        // P(W=True | R=False, S=True) = 0.90
        // P(W=True | R=False, S=False) = 0.00
        let wetgrass_cpt = BinaryCPT {
            table: vec![
                (vec![State::True,  State::True],  0.99),
                (vec![State::True,  State::False], 0.80),
                (vec![State::False, State::True],  0.90),
                (vec![State::False, State::False], 0.00),
            ],
        };
        bn.add_node("WetGrass", Box::new(wetgrass_cpt));

        // Conexiones del DAG
        bn.add_edge("Rain", "WetGrass"); // Rain → WetGrass
        bn.add_edge("Sprinkler", "WetGrass"); // Sprinkler → WetGrass

        bn
    }

    #[test]
    fn test_topological_order() {
        let bn = setup();
        let order = bn.topological_order().unwrap();

        // Una topological válida puede ser:
        // [0,1,2] o [1,0,2]
        assert!(order == vec![0, 1, 2] || order == vec![1, 0, 2]);
    }

    #[test]
    fn test_rejection_sampling_rain() {
        let bn = setup();

        // Queremos P(Rain=True)
        let query = 0;
        let evidence = HashMap::new();
        let prob = bn.rejection_sampling(&evidence, query, 10_000);

        println!("P(Rain=True) ≈ {}", prob);

        // Debería aproximar 0.2
        assert!((prob - 0.2).abs() < 0.05);
    }

    #[test]
    fn test_rejection_sampling_wetgrass_given_rain() {
        let bn = setup();

        // Query: P(WetGrass=True | Rain=True)
        let mut evidence = HashMap::new();
        evidence.insert(0, State::True); // Rain=True

        let query = 2;
        let prob = bn.rejection_sampling(&evidence, query, 10_000);

        println!("P(WetGrass=True | Rain=True) ≈ {}", prob);

        // Valor teórico esperado (aprox): 0.89
        assert!((prob - 0.89).abs() < 0.07);
    }
}
