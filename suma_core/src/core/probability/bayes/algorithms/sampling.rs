use crate::core::probability::bayes::BN_base::State;
use crate::core::probability::bayes::BayesianNetwork;
use std::collections::HashMap;
use crate::core::data_structures::graphs::{Directed, topological_sort};

pub fn rejection_sampling(
    network: &BayesianNetwork,
    evidence: &HashMap<usize, State>,
    query: usize,
    n_samples: usize,
) -> f64
{
    let topo_order = &network.topological_order().unwrap();
    let mut count_query_true = 0;
    let mut accepted_samples = 0;

    for _ in 0..n_samples {
        let mut sample = HashMap::new();
        let mut valid = true;

        for node in topo_order {
            if let Some(val) = evidence.get(node) {
                sample.insert(*node, val.clone());
            } else {
                let parent_values = network.get_parent_values(node, &sample);
                let val = network.sample_node(node, &parent_values);
                sample.insert(*node, val);
            }
        }

        // Check evidence
        for (node, val) in evidence {
            if sample[node] != *val {
                valid = false;
                break;
            }
        }

        if valid {
            accepted_samples += 1;
            if sample[&query] == State::True {
                count_query_true += 1;
            }
        }
    }

    count_query_true as f64 / accepted_samples as f64
}

#[cfg(test)]
mod tests {
    use crate::core::probability::bayes::BN_base::CPTBase;
use super::*;
    use crate::core::probability::bayes::implementations::CPTs::BinaryCPT;
    use crate::core::data_structures::dag::DAG;
    use crate::core::probability::bayes::BN_base::State;
    use std::collections::HashMap;

    #[test]
    fn test_rejection_sampling() {
        // Crear un DAG pequeño
        let mut dag = DAG::<usize>::new();
        dag.add_node(0);
        dag.add_node(1);
        dag.add_edge(0, 1);

        // Crear CPTs
        let mut cpts: HashMap<usize, Box<dyn CPTBase>> = HashMap::new();
        // Nodo 0: A sin padres, P(A=True) = 0.6
        cpts.insert(0, Box::new(BinaryCPT { table: vec![(vec![], 0.6)] }));
        // Nodo 1: B con padre A, P(B=True|A=True)=0.8, P(B=True|A=False)=0.3
        cpts.insert(1, Box::new(BinaryCPT {
            table: vec![(vec![State::True], 0.8), (vec![State::False], 0.3)]
        }));

        // Mapeo de nombres a IDs
        let mut name_to_id = HashMap::new();
        name_to_id.insert("A".to_string(), 0);
        name_to_id.insert("B".to_string(), 1);

        let network = BayesianNetwork::from_parts(dag, cpts, name_to_id);

        let evidence = HashMap::new();
        let query = 1; // Nodo B
        let n_samples = 10000;

        let prob = rejection_sampling(&network, &evidence, query, n_samples);
        println!("P(B=True) ≈ {}", prob);

        // Probabilidad exacta: 0.8*0.6 + 0.3*0.4 = 0.48 + 0.12 = 0.6
        assert!((prob - 0.6).abs() < 0.05);
    }
}
