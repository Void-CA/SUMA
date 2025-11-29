use crate::core::probability::bayes::BN_base::State;
use crate::core::probability::bayes::BayesianNetwork;
use std::collections::HashMap;

pub fn rejection_sampling(
    network: &BayesianNetwork,
    evidence: &HashMap<usize, State>,
    query: usize,
    n_samples: usize,
) -> HashMap<State, f64>
{
    let topo_order = network.topological_order().unwrap();
    let mut counts: HashMap<State, usize> = HashMap::new();
    let mut accepted_samples = 0;

    for _ in 0..n_samples {
        let mut sample = HashMap::new();
        let mut valid = true;

        for node in &topo_order {
            let parent_values = network.get_parent_values(node, &sample);
            // 1. Generar el valor muestreado (P(X|Parents))
            let generated_val = network.sample_node(node, &parent_values);

            let node_value;

            if let Some(evidence_val) = evidence.get(node) {
                // 2. Si el nodo es evidencia, verificar consistencia
                if generated_val == *evidence_val {
                    // Si es consistente, usar el valor de la evidencia para la muestra.
                    // Esto es crucial para asegurar que el resto de la red use este valor.
                    node_value = evidence_val.clone();
                } else {
                    // 3. Si no es consistente, rechazar la muestra
                    valid = false;
                    break;
                }
            } else {
                // 4. Si no es evidencia, usar el valor muestreado
                node_value = generated_val;
            }

            // Insertar el valor del nodo en la muestra
            sample.insert(*node, node_value);
        }

        if valid {
            accepted_samples += 1;
            // Clonamos el estado del nodo consultado
            let result_state = sample.get(&query).unwrap().clone();
            *counts.entry(result_state).or_insert(0) += 1;
        }
    }

    let mut distribution = HashMap::new();
    if accepted_samples > 0 {
        for (state, count) in counts {
            distribution.insert(state, count as f64 / accepted_samples as f64);
        }
    }

    distribution
}

pub fn likelihood_sampling( // Nombre mantenido por compatibilidad con WASM
   network: &BayesianNetwork,
   evidence: &HashMap<usize, State>,
   query: usize,
   n_samples: usize,
) -> HashMap<State, f64>
{
    let topo_order = network.topological_order().unwrap();
    let mut weighted_counts: HashMap<State, f64> = HashMap::new();
    let mut total_weight = 0.0;

    for _ in 0..n_samples {
        let mut sample = HashMap::new();
        let mut sample_weight = 1.0;

        for node in &topo_order {
            let parent_values = network.get_parent_values(node, &sample);
            let node_value;

            if let Some(evidence_val) = evidence.get(node) {
                // 1. Si el nodo es evidencia, forzar el valor y calcular el peso.
                node_value = evidence_val.clone();

                // 2. Ponderar la muestra: W = W * P(Evidencia | Padres)
                let prob_evidence = network.get_conditional_probability(*node, &parent_values, evidence_val.clone()).unwrap();
                sample_weight *= prob_evidence;

            } else {
                // 3. Si no es evidencia, muestrear el valor normalmente.
                node_value = network.sample_node(node, &parent_values);
            }

            // Insertar el valor del nodo (ya sea muestreado o forzado)
            sample.insert(*node, node_value);
        }

        // Usar la muestra si tiene un peso positivo
        if sample_weight > 0.0 {
            let result_state = sample.get(&query).unwrap().clone();

            // 4. Acumular el peso
            *weighted_counts.entry(result_state).or_insert(0.0) += sample_weight;
            total_weight += sample_weight;
        }
    }

    // 5. Normalizar la distribución por el peso total
    let mut distribution = HashMap::new();
    if total_weight > 0.0 {
        for (state, weight) in weighted_counts {
            distribution.insert(state, weight / total_weight);
        }
    }

    distribution
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::probability::bayes::BN_base::{CPTBase, State};
    use crate::core::probability::bayes::implementations::CPTs::BinaryCPT;
    use crate::core::data_structures::dag::DAG;
    use std::collections::HashMap;

    #[test]
    fn test_likelihood_sampling() {
        // Crear un DAG pequeño
        let mut dag = DAG::<usize>::new();
        dag.add_node(0);
        dag.add_node(1);
        dag.add_edge(0, 1).unwrap();

        // Crear CPTs - versión corregida
        let mut cpts: HashMap<usize, Box<dyn CPTBase>> = HashMap::new();

        // Nodo 0: A sin padres, P(A=True) = 0.6
        // Para BinaryCPT con no padres, solo necesitamos P(True), P(False) se infiere como 1 - P(True)
        cpts.insert(0, Box::new(BinaryCPT::new_no_parents(
            vec![State::True, State::False], // possible_values
            vec![0.6] // SOLO una probabilidad para True
        )));

        // Nodo 1: B con padre A
        cpts.insert(1, Box::new(BinaryCPT::new_with_parents(
            vec![
                vec![State::True],  // A=True
                vec![State::False], // A=False
            ],
            vec![
                // Para A=True: P(B=True) = 0.8
                HashMap::from([(State::True, 0.8)]),
                // Para A=False: P(B=True) = 0.3
                HashMap::from([(State::True, 0.3)]),
            ],
            vec![State::True, State::False] // possible_values
        )));

        // Mapeo de nombres a IDs
        let mut name_to_id = HashMap::new();
        name_to_id.insert("A".to_string(), 0);
        name_to_id.insert("B".to_string(), 1);

        let network = BayesianNetwork::from_parts(dag, cpts, name_to_id);

        let evidence = HashMap::new();
        let query = 1; // Nodo B
        let n_samples = 10000;

        let distribution = likelihood_sampling(&network, &evidence, query, n_samples);

        let prob_b_true = *distribution.get(&State::True).unwrap_or(&0.0);
        println!("P(B=True) ≈ {}", prob_b_true);

        // Probabilidad exacta: P(B=True) = P(B=True|A=True)*P(A=True) + P(B=True|A=False)*P(A=False)
        // = 0.8*0.6 + 0.3*0.4 = 0.48 + 0.12 = 0.6
        assert!((prob_b_true - 0.6).abs() < 0.05, "Probabilidad estimada: {}, esperada: ~0.6", prob_b_true);
    }

    #[test]
    fn test_likelihood_sampling_with_evidence() {
        // Test con evidencia: P(B=True|A=True)
        let mut dag = DAG::<usize>::new();
        dag.add_node(0);
        dag.add_node(1);
        dag.add_edge(0, 1).unwrap();

        let mut cpts: HashMap<usize, Box<dyn CPTBase>> = HashMap::new();

        cpts.insert(0, Box::new(BinaryCPT::new_no_parents(
            vec![State::True, State::False],
            vec![0.6] // Solo P(A=True)
        )));

        cpts.insert(1, Box::new(BinaryCPT::new_with_parents(
            vec![
                vec![State::True],
                vec![State::False],
            ],
            vec![
                HashMap::from([(State::True, 0.8)]), // P(B=True|A=True)
                HashMap::from([(State::True, 0.3)]), // P(B=True|A=False)
            ],
            vec![State::True, State::False]
        )));

        let mut name_to_id = HashMap::new();
        name_to_id.insert("A".to_string(), 0);
        name_to_id.insert("B".to_string(), 1);

        let network = BayesianNetwork::from_parts(dag, cpts, name_to_id);

        // Evidencia: A = True
        let mut evidence = HashMap::new();
        evidence.insert(0, State::True);

        let query = 1; // Nodo B
        let n_samples = 10000;

        let distribution = likelihood_sampling(&network, &evidence, query, n_samples);

        let prob_b_true = *distribution.get(&State::True).unwrap_or(&0.0);
        println!("P(B=True|A=True) ≈ {}", prob_b_true);

        // Probabilidad exacta: P(B=True|A=True) = 0.8
        assert!((prob_b_true - 0.8).abs() < 0.05, "Probabilidad estimada: {}, esperada: ~0.8", prob_b_true);
    }

    #[test]
    fn test_likelihood_sampling_complex_evidence() {
        // Test con evidencia en el nodo hijo: P(A=True|B=True)
        let mut dag = DAG::<usize>::new();
        dag.add_node(0);
        dag.add_node(1);
        dag.add_edge(0, 1).unwrap();

        let mut cpts: HashMap<usize, Box<dyn CPTBase>> = HashMap::new();

        cpts.insert(0, Box::new(BinaryCPT::new_no_parents(
            vec![State::True, State::False],
            vec![0.6] // Solo P(A=True)
        )));

        cpts.insert(1, Box::new(BinaryCPT::new_with_parents(
            vec![
                vec![State::True],
                vec![State::False],
            ],
            vec![
                HashMap::from([(State::True, 0.8)]), // P(B=True|A=True)
                HashMap::from([(State::True, 0.3)]), // P(B=True|A=False)
            ],
            vec![State::True, State::False]
        )));

        let mut name_to_id = HashMap::new();
        name_to_id.insert("A".to_string(), 0);
        name_to_id.insert("B".to_string(), 1);

        let network = BayesianNetwork::from_parts(dag, cpts, name_to_id);

        // Evidencia: B = True (nodo hijo)
        let mut evidence = HashMap::new();
        evidence.insert(1, State::True);

        let query = 0; // Nodo A
        let n_samples = 10000;

        let distribution = likelihood_sampling(&network, &evidence, query, n_samples);

        let prob_a_true = *distribution.get(&State::True).unwrap_or(&0.0);
        println!("P(A=True|B=True) ≈ {}", prob_a_true);

        // Probabilidad exacta por Bayes:
        // P(A=True|B=True) = P(B=True|A=True)*P(A=True) / P(B=True)
        // = (0.8 * 0.6) / 0.6 = 0.48 / 0.6 = 0.8
        assert!((prob_a_true - 0.8).abs() < 0.05, "Probabilidad estimada: {}, esperada: ~0.8", prob_a_true);
    }

    #[test]
    fn test_rejection_sampling_three_nodes() {
        // Red más compleja: A -> B -> C
        let mut dag = DAG::<usize>::new();
        dag.add_node(0);
        dag.add_node(1);
        dag.add_node(2);
        dag.add_edge(0, 1).unwrap();
        dag.add_edge(1, 2).unwrap();

        let mut cpts: HashMap<usize, Box<dyn CPTBase>> = HashMap::new();

        // A: P(A=True)=0.7
        cpts.insert(0, Box::new(BinaryCPT::new_no_parents(
            vec![State::True, State::False],
            vec![0.7] // Solo P(A=True)
        )));

        // B: P(B=True|A=True)=0.9, P(B=True|A=False)=0.2
        cpts.insert(1, Box::new(BinaryCPT::new_with_parents(
            vec![
                vec![State::True],
                vec![State::False],
            ],
            vec![
                HashMap::from([(State::True, 0.9)]), // P(B=True|A=True)
                HashMap::from([(State::True, 0.2)]), // P(B=True|A=False)
            ],
            vec![State::True, State::False]
        )));

        // C: P(C=True|B=True)=0.95, P(C=True|B=False)=0.1
        cpts.insert(2, Box::new(BinaryCPT::new_with_parents(
            vec![
                vec![State::True],
                vec![State::False],
            ],
            vec![
                HashMap::from([(State::True, 0.95)]), // P(C=True|B=True)
                HashMap::from([(State::True, 0.1)]),  // P(C=True|B=False)
            ],
            vec![State::True, State::False]
        )));

        let mut name_to_id = HashMap::new();
        name_to_id.insert("A".to_string(), 0);
        name_to_id.insert("B".to_string(), 1);
        name_to_id.insert("C".to_string(), 2);

        let network = BayesianNetwork::from_parts(dag, cpts, name_to_id);

        let evidence = HashMap::new();
        let query = 2; // Nodo C
        let n_samples = 10000;

        let distribution = likelihood_sampling(&network, &evidence, query, n_samples);

        let prob_c_true = *distribution.get(&State::True).unwrap_or(&0.0);
        println!("P(C=True) ≈ {}", prob_c_true);

        // Probabilidad exacta:
        // P(C=True) = P(C=True|B=True)P(B=True) + P(C=True|B=False)P(B=False)
        // P(B=True) = P(B=True|A=True)P(A=True) + P(B=True|A=False)P(A=False)
        // = 0.9*0.7 + 0.2*0.3 = 0.63 + 0.06 = 0.69
        // P(C=True) = 0.95*0.69 + 0.1*0.31 = 0.6555 + 0.031 = 0.6865
        let expected_prob = 0.95 * (0.9 * 0.7 + 0.2 * 0.3) + 0.1 * (0.1 * 0.7 + 0.8 * 0.3);
        assert!((prob_c_true - expected_prob).abs() < 0.05,
                "Probabilidad estimada: {}, esperada: ~{}", prob_c_true, expected_prob);
    }
}