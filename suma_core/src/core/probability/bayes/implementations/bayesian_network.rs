use crate::core::probability::bayes::BinaryCPT;
use crate::core::probability::bayes::DiscreteCPT;
use std::hash::Hash;
use std::collections::HashMap;
use rust_xlsxwriter::TableFunction;
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

    pub fn add_edge_by_id(&mut self, parent: usize, child: usize) {
        self.dag.add_edge(parent, child);
    }

    pub fn get_id_from_name(&self, name: &str) -> Option<usize> {
        self.name_to_id.get(name).copied()
    }

    pub fn get_name_from_id(&self, node_id: usize) -> Option<&String> {
        self.id_to_name.get(&node_id)
    }

    fn next_node_id(&self) -> usize {
        self.cpts.len()
    }

    pub fn add_node(
        &mut self,
        name: &str,
        cpt: Box<dyn CPTBase>
    ) -> Result<usize, &'static str> {
        if self.name_to_id.contains_key(name) {
            return Err("Node with this name already exists.");
        }

        let node_id = self.next_node_id();
        self.dag.add_node(node_id);
        self.cpts.insert(node_id, cpt);
        self.name_to_id.insert(name.to_string(), node_id);
        self.id_to_name.insert(node_id, name.to_string());

        Ok(node_id)
    }

    pub fn add_binary_node(
        &mut self,
        name: &str,
        parent_names: Vec<&str>,
        prob_true_given_parents: Vec<(Vec<bool>, f64)>
    ) -> Result<usize, String> {

        let mut parent_ids = Vec::new();
        for p_name in &parent_names {
            match self.get_id_from_name(p_name) {
                Some(id) => parent_ids.push(id),
                None => return Err(format!("Parent node '{}' not found.", p_name)),
            }
        }

        // 2. CONSTRUCCIÓN INTERNA DEL CPT: Usar BinaryCPT
        let internal_table: Vec<(Vec<State>, f64)> = prob_true_given_parents
            .into_iter()
            .map(|(bool_parents, prob)| {
                let state_parents: Vec<State> = bool_parents
                    .into_iter()
                    // Mapear bool a tu State binario (asumiendo que State tiene True/False)
                    .map(|b| if b { State::True } else { State::False })
                    .collect();
                (state_parents, prob)
            })
            .collect();


        let cpt = BinaryCPT { table: internal_table };

        // 3. Agregar el nodo y su CPT
        let node_id = self.add_node(name, Box::new(cpt)).map_err(|e| e.to_string())?;

        // 4. Agregar las aristas
        for p_id in parent_ids {
            self.add_edge_by_id(p_id, node_id);
        }

        Ok(node_id)
    }

    pub fn add_discrete_node(
        &mut self,
        name: &str,
        parent_names: Vec<&str>,
        possible_values: Vec<&str>, // Valores que toma este nodo (ej: "A", "B", "C")
        table_data: HashMap<Vec<&str>, HashMap<&str, f64>>
    ) -> Result<usize, String> {

        let mut parent_ids = Vec::new();
        for p_name in &parent_names {
            match self.get_id_from_name(p_name) {
                Some(id) => parent_ids.push(id),
                None => return Err(format!("Parent node '{}' not found.", p_name)),
            }
        }

        let internal_possible_values: Vec<State> = possible_values
            .into_iter()
            .map(|s| State::from_str(s))
            .collect();

        // b) Construir la tabla interna: HashMap<Vec<State>, HashMap<State, f64>>
        let mut internal_table = HashMap::new();

        for (parent_vals_str, distribution_str) in table_data.iter() {
            // Mapear combinación de padres (strings) a Vec<State>
            let parent_vals_state: Vec<State> = parent_vals_str
                .into_iter()
                .map(|s| State::from_str(s))
                .collect();

            // Mapear la distribución (strings) a HashMap<State, f64>
            let distribution_state: HashMap<State, f64> = distribution_str
                .iter()
                .map(|(val_str, prob)| (State::from_str(val_str), *prob))
                .collect();

            // Validación de Suma de Probabilidades (Crucial para CPTs)
            let sum_prob: f64 = distribution_state.values().sum();
            if (sum_prob - 1.0).abs() > 1e-6 { // Usar tolerancia para floats
                return Err(format!("Probabilities for parent combination {:?} do not sum to 1.0 (Sum: {}).",
                                   &parent_vals_str, &sum_prob));
            }

            internal_table.insert(parent_vals_state, distribution_state);
        }

        let cpt = DiscreteCPT {
            node_possible_values: internal_possible_values,
            table: internal_table
        };

        // 3. Agregar el nodo y su CPT
        let node_id = self.add_node(name, Box::new(cpt)) // Usa el método base 'add_node'
            .map_err(|e| e.to_string())?;

        // 4. Agregar las aristas
        for p_id in parent_ids {
            self.add_edge_by_id(p_id, node_id);
        }

        Ok(node_id)
    }



    pub fn print_ids(&self) {
        for (name, id) in &self.name_to_id {
            println!("Node Name: {}, ID: {}", name, id);
        }
    }

    pub fn make_states(&self, names: &[&str]) -> Vec<State> {
        names.iter()
            .map(|&n| State::Value(n.to_string()))
            .collect()
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
    fn setup() -> Result<BayesianNetwork, String> {
        let mut bn = BayesianNetwork::new();

        // --- 1. Nodo Rain (Binario, sin padres) ---
        // P(Rain=True) = 0.2
        let rain_cpt_data: Vec<(Vec<bool>, f64)> = vec![
            (vec![], 0.2),
        ];

        bn.add_binary_node(
            "Rain",
            vec![], // parent_names: No tiene padres
            rain_cpt_data
        )?;

        // --- 2. Nodo Sprinkler (Binario, dependiente de Rain) ---
        // P(S=True | R=True) = 0.01
        // P(S=True | R=False) = 0.40
        let sprinkler_cpt_data: Vec<(Vec<bool>, f64)> = vec![
            // Padres [Rain]
            (vec![true], 0.01),  // P(S=True | R=True)
            (vec![false], 0.40), // P(S=True | R=False)
        ];

        // Usamos add_binary_node
        bn.add_binary_node(
            "Sprinkler",
            vec!["Rain"], // parent_names: Depende de Rain
            sprinkler_cpt_data
        )?;

        // --- 3. Nodo WetGrass (Binario, dependiente de Rain y Sprinkler) ---
        // Los padres deben estar en el mismo orden que en la lista de `parent_names`: ["Rain", "Sprinkler"]

        // P(W=True | R, S)
        let wetgrass_cpt_data: Vec<(Vec<bool>, f64)> = vec![
            // Padres [Rain, Sprinkler]
            (vec![true, true], 0.99),   // P(W=True | R=True, S=True)
            (vec![true, false], 0.80),  // P(W=True | R=True, S=False)
            (vec![false, true], 0.90),  // P(W=True | R=False, S=True)
            (vec![false, false], 0.00), // P(W=True | R=False, S=False)
        ];

        // Usamos add_binary_node
        bn.add_binary_node(
            "WetGrass",
            vec!["Rain", "Sprinkler"], // parent_names: Orden de los padres
            wetgrass_cpt_data
        )?;


        Ok(bn)
    }

    #[test]
    fn test_topological_order() {
        let bn = setup().unwrap();
        let order = bn.topological_order().unwrap();

        // Una topological válida puede ser:
        // [0,1,2] o [1,0,2]
        assert!(order == vec![0, 1, 2] || order == vec![1, 0, 2]);
    }

    #[test]
    fn test_rejection_sampling_rain() {
        let bn = setup().unwrap();

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
        let bn = setup().unwrap();

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
