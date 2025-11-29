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
    ) -> HashMap<State, f64> {
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
            let rand_val: f64 = super::super::super::utils::random::random_f64();
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
    use std::collections::HashMap;

    // Construye una red bayesiana simple para tests
    fn setup_simple_network() -> Result<BayesianNetwork, String> {
        let mut bn = BayesianNetwork::new();

        // Nodo Rain (sin padres)
        bn.add_binary_node(
            "Rain",
            vec![],
            vec![(vec![], 0.2)]
        )?;

        // Nodo Sprinkler (depende de Rain)
        bn.add_binary_node(
            "Sprinkler",
            vec!["Rain"],
            vec![
                (vec![true], 0.01),
                (vec![false], 0.40),
            ]
        )?;

        // Nodo WetGrass (depende de Rain y Sprinkler)
        bn.add_binary_node(
            "WetGrass",
            vec!["Rain", "Sprinkler"],
            vec![
                (vec![true, true], 0.99),
                (vec![true, false], 0.80),
                (vec![false, true], 0.90),
                (vec![false, false], 0.00),
            ]
        )?;

        Ok(bn)
    }

    // Setup para tests de nodos discretos
    fn setup_discrete_network() -> Result<BayesianNetwork, String> {
        let mut bn = BayesianNetwork::new();

        // Nodo discreto: Difficulty del curso
        bn.add_discrete_node(
            "Difficulty",
            vec![],
            vec!["Easy", "Hard"],
            HashMap::from([
                (vec![], HashMap::from([("Easy", 0.6), ("Hard", 0.4)]))
            ])
        )?;

        // Nodo discreto: Intelligence del estudiante
        bn.add_discrete_node(
            "Intelligence",
            vec![],
            vec!["Low", "High"],
            HashMap::from([
                (vec![], HashMap::from([("Low", 0.7), ("High", 0.3)]))
            ])
        )?;

        // Nodo discreto: Grade (depende de Difficulty e Intelligence)
        bn.add_discrete_node(
            "Grade",
            vec!["Difficulty", "Intelligence"],
            vec!["A", "B", "C"],
            HashMap::from([
                (vec!["Easy", "High"], HashMap::from([("A", 0.8), ("B", 0.15), ("C", 0.05)])),
                (vec!["Easy", "Low"], HashMap::from([("A", 0.5), ("B", 0.3), ("C", 0.2)])),
                (vec!["Hard", "High"], HashMap::from([("A", 0.6), ("B", 0.25), ("C", 0.15)])),
                (vec!["Hard", "Low"], HashMap::from([("A", 0.2), ("B", 0.4), ("C", 0.4)])),
            ])
        )?;

        Ok(bn)
    }

    // Tests básicos de estructura
    #[test]
    fn test_network_creation() {
        let bn = BayesianNetwork::new();
        assert_eq!(bn.get_nodes().len(), 0);
        assert_eq!(bn.get_edges().len(), 0);
    }

    #[test]
    fn test_add_binary_nodes() {
        let bn = setup_simple_network().unwrap();

        assert_eq!(bn.get_nodes().len(), 3);
        assert_eq!(bn.get_edges().len(), 3);

        // Verificar que los nodos existen
        assert!(bn.get_id_from_name("Rain").is_some());
        assert!(bn.get_id_from_name("Sprinkler").is_some());
        assert!(bn.get_id_from_name("WetGrass").is_some());
    }

    #[test]
    fn test_add_discrete_nodes() {
        let bn = setup_discrete_network().unwrap();

        assert_eq!(bn.get_nodes().len(), 3);
        assert_eq!(bn.get_edges().len(), 2);

        // Verificar nombres
        assert!(bn.get_id_from_name("Difficulty").is_some());
        assert!(bn.get_id_from_name("Intelligence").is_some());
        assert!(bn.get_id_from_name("Grade").is_some());
    }

    #[test]
    fn test_duplicate_node_name() {
        let mut bn = BayesianNetwork::new();

        let result1 = bn.add_binary_node("Rain", vec![], vec![(vec![], 0.2)]);
        assert!(result1.is_ok());

        let result2 = bn.add_binary_node("Rain", vec![], vec![(vec![], 0.5)]);
        assert!(result2.is_err());
    }

    #[test]
    fn test_nonexistent_parent() {
        let mut bn = BayesianNetwork::new();

        let result = bn.add_binary_node(
            "Child",
            vec!["NonexistentParent"],
            vec![(vec![true], 0.5)]
        );

        assert!(result.is_err());
    }

    // Tests de estructura del grafo
    #[test]
    fn test_topological_order() {
        let bn = setup_simple_network().unwrap();
        let order = bn.topological_order().unwrap();

        // Verificar que el orden es válido
        assert_eq!(order.len(), 3);

        // Rain y Sprinkler deben venir antes que WetGrass
        let wetgrass_id = bn.get_id_from_name("WetGrass").unwrap();
        let rain_id = bn.get_id_from_name("Rain").unwrap();
        let sprinkler_id = bn.get_id_from_name("Sprinkler").unwrap();

        let wetgrass_pos = order.iter().position(|&x| x == wetgrass_id).unwrap();
        let rain_pos = order.iter().position(|&x| x == rain_id).unwrap();
        let sprinkler_pos = order.iter().position(|&x| x == sprinkler_id).unwrap();

        assert!(rain_pos < wetgrass_pos);
        assert!(sprinkler_pos < wetgrass_pos);
    }

    #[test]
    fn test_get_parents_children() {
        let bn = setup_simple_network().unwrap();
        let wetgrass_id = bn.get_id_from_name("WetGrass").unwrap();
        let rain_id = bn.get_id_from_name("Rain").unwrap();
        let sprinkler_id = bn.get_id_from_name("Sprinkler").unwrap();

        let wetgrass_parents = bn.get_parents(wetgrass_id);
        assert_eq!(wetgrass_parents.len(), 2);
        assert!(wetgrass_parents.contains(&rain_id));
        assert!(wetgrass_parents.contains(&sprinkler_id));

        let rain_children = bn.get_children(rain_id);
        assert_eq!(rain_children.len(), 2); // Sprinkler y WetGrass
    }

    // Tests de CPT
    #[test]
    fn test_get_cpt() {
        let bn = setup_simple_network().unwrap();
        let rain_id = bn.get_id_from_name("Rain").unwrap();

        let cpt = bn.get_cpt(rain_id);
        assert!(cpt.is_some());
    }

    #[test]
    fn test_cpt_probability_calculation() {
        let bn = setup_simple_network().unwrap();
        let sprinkler_id = bn.get_id_from_name("Sprinkler").unwrap();

        let cpt = bn.get_cpt(sprinkler_id).unwrap();

        // P(Sprinkler=True | Rain=True) debería ser 0.01
        let prob = cpt.get_probability(&[State::True], State::True);
        assert!(prob.is_some());
        assert!((prob.unwrap() - 0.01).abs() < 1e-6);

        // P(Sprinkler=True | Rain=False) debería ser 0.40
        let prob = cpt.get_probability(&[State::False], State::True);
        assert!(prob.is_some());
        assert!((prob.unwrap() - 0.40).abs() < 1e-6);
    }

    // Tests de sampling
    #[test]
    fn test_sample_node() {
        let bn = setup_simple_network().unwrap();
        let rain_id = bn.get_id_from_name("Rain").unwrap();

        // Samplear sin padres
        let sample = bn.sample_node(&rain_id, &[]);
        assert!(matches!(sample, State::True | State::False));
    }

    #[test]
    fn test_get_parent_values() {
        let bn = setup_simple_network().unwrap();
        let wetgrass_id = bn.get_id_from_name("WetGrass").unwrap();

        let mut sample = HashMap::new();
        sample.insert(bn.get_id_from_name("Rain").unwrap(), State::True);
        sample.insert(bn.get_id_from_name("Sprinkler").unwrap(), State::False);

        let parent_values = bn.get_parent_values(&wetgrass_id, &sample);

        // En lugar de verificar el orden exacto, verificar que contiene los valores correctos
        assert_eq!(parent_values.len(), 2);
        assert!(parent_values.contains(&State::True));
        assert!(parent_values.contains(&State::False));

        // O verificar usando conjuntos
        let expected_values: std::collections::HashSet<_> =
            [State::True, State::False].iter().cloned().collect();
        let actual_values: std::collections::HashSet<_> =
            parent_values.iter().cloned().collect();
        assert_eq!(expected_values, actual_values);
    }

    // Tests de eliminación de nodos
    #[test]
    fn test_remove_node() {
        let mut bn = setup_simple_network().unwrap();
        let rain_id = bn.get_id_from_name("Rain").unwrap();

        assert_eq!(bn.get_nodes().len(), 3);
        assert_eq!(bn.get_edges().len(), 3);

        let result = bn.remove_node(rain_id);
        assert!(result.is_some());

        assert_eq!(bn.get_nodes().len(), 2);
        // Al eliminar Rain, deberían eliminarse las aristas hacia Sprinkler y WetGrass
        assert_eq!(bn.get_edges().len(), 1); // Solo queda Sprinkler -> WetGrass
    }

    #[test]
    fn test_remove_nonexistent_node() {
        let mut bn = setup_simple_network().unwrap();

        let result = bn.remove_node(999); // ID que no existe
        assert!(result.is_none());
    }

    // Tests de mapeo nombre-ID
    #[test]
    fn test_name_id_mapping() {
        let bn = setup_simple_network().unwrap();

        let rain_id = bn.get_id_from_name("Rain").unwrap();
        let rain_name = bn.get_name_from_id(rain_id).unwrap();

        assert_eq!(rain_name, "Rain");
        assert_eq!(bn.get_id_from_name("Rain").unwrap(), rain_id);
    }

    #[test]
    fn test_nonexistent_name_mapping() {
        let bn = setup_simple_network().unwrap();

        assert!(bn.get_id_from_name("Nonexistent").is_none());
        assert!(bn.get_name_from_id(999).is_none());
    }

    // Tests de validación de probabilidades
    #[test]
    fn test_invalid_probability_sum() {
        let mut bn = BayesianNetwork::new();

        // Primero agregar el nodo padre
        bn.add_discrete_node(
            "Parent",
            vec![],
            vec!["A", "B"],
            HashMap::from([(vec![], HashMap::from([("A", 0.6), ("B", 0.3)]))]) // Suma 0.9, debería fallar
        ).unwrap_err(); // Esto debería fallar porque la suma no es 1.0
    }

    // Test de make_states
    #[test]
    fn test_make_states() {
        let bn = BayesianNetwork::new();
        let states = bn.make_states(&["A", "B", "C"]);

        assert_eq!(states.len(), 3);
        assert!(matches!(states[0], State::Value(ref s) if s == "A"));
        assert!(matches!(states[1], State::Value(ref s) if s == "B"));
        assert!(matches!(states[2], State::Value(ref s) if s == "C"));
    }

    // Test de rejection sampling básico
    #[test]
    fn test_rejection_sampling_basic() {
        let bn = setup_simple_network().unwrap();
        let rain_id = bn.get_id_from_name("Rain").unwrap();

        let mut evidence = HashMap::new();
        evidence.insert(rain_id, State::True);

        let wetgrass_id = bn.get_id_from_name("WetGrass").unwrap();
        let distribution = bn.rejection_sampling(&evidence, wetgrass_id, 1000);

        // Verificar que devuelve una distribución válida
        assert!(!distribution.is_empty());
        let total_prob: f64 = distribution.values().sum();
        assert!((total_prob - 1.0).abs() < 0.1); // Permitir cierto margen de error
    }

    // Test de construcción desde partes
    #[test]
    fn test_from_parts() {
        let mut dag = DAG::new();
        dag.add_node(0);
        dag.add_node(1);
        dag.add_edge(0, 1);

        let mut cpts = HashMap::new();
        cpts.insert(0, Box::new(BinaryCPT { table: vec![(vec![], 0.5)] }) as Box<dyn CPTBase>);
        cpts.insert(1, Box::new(BinaryCPT { table: vec![(vec![State::True], 0.8), (vec![State::False], 0.2)] }));

        let mut name_to_id = HashMap::new();
        name_to_id.insert("A".to_string(), 0);
        name_to_id.insert("B".to_string(), 1);

        let bn = BayesianNetwork::from_parts(dag, cpts, name_to_id);

        assert_eq!(bn.get_nodes().len(), 2);
        assert_eq!(bn.get_edges().len(), 1);
        assert_eq!(bn.get_id_from_name("A"), Some(0));
        assert_eq!(bn.get_name_from_id(1).unwrap(), "B");
    }

    // Test de print_ids (aunque es difícil testear output, al menos verificar que no panic)
    #[test]
    fn test_print_ids_no_panic() {
        let bn = setup_simple_network().unwrap();
        bn.print_ids(); // Solo verificamos que no cause panic
    }
}