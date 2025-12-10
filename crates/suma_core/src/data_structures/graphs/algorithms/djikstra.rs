use crate::data_structures::graphs::{WeightedGraph, graph_base::GraphBase, weighted::Weight};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use std::fmt::Debug;
use num_traits::{Num, Zero, Bounded};
use ordered_float::OrderedFloat;

pub fn dijkstra_algorithm<G>(graph: &G, source: G::NodeId) -> HashMap<G::NodeId, G::EdgeData>
where
    G: WeightedGraph,
    G::EdgeData: Weight,
    G::NodeId: Clone + Eq + Ord, <G as GraphBase>::EdgeData: Debug, <G as GraphBase>::NodeId: Debug
{
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    println!("Graph nodes: {:?}", graph.nodes());
    for node in graph.nodes() {
        if node == source {
            distances.insert(node.clone(), G::EdgeData::zero());
            heap.push((Reverse(G::EdgeData::zero()), node.clone()));
        } else {
            distances.insert(node.clone(), G::EdgeData::inf());
        }
    }

    println!("Heap: {:?}", heap);
    while let Some((Reverse(current_dist), node)) = heap.pop() {
        // Si este valor no es el más reciente, lo saltamos
        if current_dist > distances[&node] {
            continue;
        }

        for neighbor in graph.neighbors(node.clone()) {
            if let Some(weight) = graph.edge_weight(node.clone(), neighbor.clone()) {
                let new_dist = current_dist + weight;

                if new_dist < distances[&neighbor] {
                    distances.insert(neighbor.clone(), new_dist.clone());
                    heap.push((Reverse(new_dist), neighbor));
                }
            }
        }
    }

    distances
}

pub fn dijkstra_path<G>(
    graph: &G,
    source: G::NodeId,
    target: G::NodeId,
) -> Option<(Vec<G::NodeId>, G::EdgeData)>
where
    G: WeightedGraph,
    G::EdgeData: Weight,
    G::NodeId: Clone + Eq + Ord,
{
    let mut distances = HashMap::new();
    let mut previous: HashMap<G::NodeId, G::NodeId> = HashMap::new();
    let mut heap = BinaryHeap::new();

    // Inicialización
    for node in graph.nodes() {
        if node == source {
            distances.insert(node.clone(), G::EdgeData::zero());
            heap.push((Reverse(G::EdgeData::zero()), node.clone()));
        } else {
            distances.insert(node.clone(), G::EdgeData::inf());
        }
    }

    while let Some((Reverse(current_dist), node)) = heap.pop() {
        if node == target {
            // Reconstrucción del camino
            let mut path = Vec::new();
            let mut current = target.clone();

            while current != source {
                path.push(current.clone());
                current = previous.get(&current)?.clone();
            }

            path.push(source.clone());
            path.reverse();

            return Some((path, current_dist));
        }

        if current_dist > distances[&node] {
            continue;
        }

        for neighbor in graph.neighbors(node.clone()) {
            if let Some(weight) = graph.edge_weight(node.clone(), neighbor.clone()) {
                let new_dist = current_dist + weight;

                if new_dist < distances[&neighbor] {
                    distances.insert(neighbor.clone(), new_dist.clone());
                    previous.insert(neighbor.clone(), node.clone());
                    heap.push((Reverse(new_dist), neighbor));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data_structures::graphs::UndirectedWeightedGraph;

    #[test]
    fn test_dijkstra_algorithm() {
        use ordered_float::OrderedFloat;

        type EdgeData = OrderedFloat<f64>;
        let mut graph: UndirectedWeightedGraph<i32, EdgeData> = UndirectedWeightedGraph::new();

        graph.add_edge(1, 2, EdgeData::from(10.0));
        graph.add_edge(1, 3, EdgeData::from(5.0));
        graph.add_edge(2, 3, EdgeData::from(2.0));
        graph.add_edge(1, 4, EdgeData::from(10.0));

        let distances = dijkstra_algorithm(&graph, 3);

        assert_eq!(distances.get(&3), Some(&OrderedFloat(0.0)));
        assert_eq!(distances.get(&1), Some(&OrderedFloat(5.0)));
        assert_eq!(distances.get(&2), Some(&OrderedFloat(2.0)));
        assert_eq!(distances.get(&4), Some(&OrderedFloat(15.0)));
    }


    #[test]
    fn test_dijkstra_path_basic() {
        type EdgeData = OrderedFloat<f64>;

        let mut graph: UndirectedWeightedGraph<i32, EdgeData> = UndirectedWeightedGraph::new();

        // Grafo:
        // 1 --10-- 2
        // 1 -- 5-- 3
        // 3 -- 2-- 2
        // 1 --10-- 4

        graph.add_edge(1, 2, EdgeData::from(10.0));
        graph.add_edge(1, 3, EdgeData::from(5.0));
        graph.add_edge(3, 2, EdgeData::from(2.0));
        graph.add_edge(1, 4, EdgeData::from(10.0));

        // Shortest path de 1 → 2 es: 1 → 3 → 2
        // Distancia = 5.0 + 2.0 = 7.0

        let result = dijkstra_path(&graph, 1, 2)
            .expect("Path should exist");

        let (path, dist) = result;

        assert_eq!(path, vec![1, 3, 2]);
        assert_eq!(dist, EdgeData::from(7.0));
    }
}


