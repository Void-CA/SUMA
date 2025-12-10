use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use num_traits::{Bounded, Num, Zero};
use crate::data_structures::graphs::weighted::Weight;
use crate::data_structures::graphs::WeightedGraph;

pub fn a_star_algorithm<G, F>(
    graph: &G,
    start: G::NodeId,
    goal: G::NodeId,
    heuristic: F,
) -> Option<(Vec<G::NodeId>, G::EdgeData)>
where
    G: WeightedGraph,
    G::EdgeData: Weight,
    G::NodeId: Clone + Eq + std::hash::Hash + Ord,
    F: Fn(&G::NodeId, &G::NodeId) -> G::EdgeData,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<G::NodeId, G::NodeId> = HashMap::new();
    let mut g_score: HashMap<G::NodeId, G::EdgeData> = HashMap::new();
    let mut closed_set: HashSet<G::NodeId> = HashSet::new();

    for node in graph.nodes() {
        g_score.insert(node.clone(), G::EdgeData::inf());
    }
    g_score.insert(start.clone(), G::EdgeData::zero());
    open_set.push((Reverse(heuristic(&start, &goal)), start.clone()));

    while let Some((Reverse(_f), current)) = open_set.pop() {
        if current == goal {
            // Reconstruir camino
            let mut path = Vec::new();
            let mut node = goal.clone();
            while let Some(prev) = came_from.get(&node) {
                path.push(node.clone());
                node = prev.clone();
            }
            path.push(start.clone());
            path.reverse();
            return Some((path, g_score[&goal]));
        }

        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current.clone());

        for neighbor in graph.neighbors(current.clone()) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            if let Some(edge_w) = graph.edge_weight(current.clone(), neighbor.clone()) {
                let tentative_g = g_score[&current] + edge_w;
                if tentative_g < *g_score.get(&neighbor).unwrap_or(&G::EdgeData::inf()) {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g.clone());
                    let f_score = tentative_g + heuristic(&neighbor, &goal);
                    open_set.push((Reverse(f_score), neighbor.clone()));
                }
            }
        }
    }

    None
}

pub fn a_star_traversal<G, F>(
    graph: &G,
    start: G::NodeId,
    goal: G::NodeId,
    heuristic: F,
) -> Option<(Vec<G::NodeId>, G::EdgeData, Vec<G::NodeId>)>
where
    G: WeightedGraph,
    G::EdgeData: Weight + std::fmt::Debug,
    G::NodeId: Clone + Eq + std::hash::Hash + Ord + std::fmt::Debug,
    F: Fn(&G::NodeId, &G::NodeId) -> G::EdgeData,
{
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::cmp::Reverse;

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<G::NodeId, G::NodeId> = HashMap::new();
    let mut g_score: HashMap<G::NodeId, G::EdgeData> = HashMap::new();
    let mut closed_set: HashSet<G::NodeId> = HashSet::new();
    let mut traversal_order = Vec::new();

    // Inicializamos g_score en infinito
    for node in graph.nodes() {
        g_score.insert(node.clone(), G::EdgeData::inf());
    }
    g_score.insert(start.clone(), G::EdgeData::zero());
    open_set.push((Reverse(heuristic(&start, &goal)), start.clone()));

    while let Some((Reverse(_f), current)) = open_set.pop() {
        traversal_order.push(current.clone());

        if current == goal {
            // Reconstrucción del camino
            let mut path = Vec::new();
            let mut node = goal.clone();
            while let Some(prev) = came_from.get(&node) {
                path.push(node.clone());
                node = prev.clone();
            }
            path.push(start.clone());
            path.reverse();
            return Some((path, g_score[&goal], traversal_order));
        }

        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current.clone());

        for neighbor in graph.neighbors(current.clone()) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            if let Some(edge_w) = graph.edge_weight(current.clone(), neighbor.clone()) {
                let tentative_g = g_score[&current] + edge_w;
                if tentative_g < *g_score.get(&neighbor).unwrap_or(&G::EdgeData::inf()) {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g.clone());
                    let f_score = tentative_g + heuristic(&neighbor, &goal);
                    open_set.push((Reverse(f_score), neighbor.clone()));
                }
            }
        }
    }

    None
}



#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;
    use crate::data_structures::graphs::{UndirectedWeightedGraph, GraphBase};

    fn setup_graph_int() -> (UndirectedWeightedGraph<&'static str, i32>, Vec<usize>) {
        let mut graph = UndirectedWeightedGraph::new();
        let n0 = graph.base.add_node("A");
        let n1 = graph.base.add_node("B");
        let n2 = graph.base.add_node("C");
        let n3 = graph.base.add_node("D");

        graph.add_edge(n0, n1, 1);
        graph.add_edge(n1, n2, 2);
        graph.add_edge(n0, n2, 4);
        graph.add_edge(n2, n3, 1);

        (graph, vec![n0, n1, n2, n3])
    }

    fn setup_graph_float() -> (UndirectedWeightedGraph<&'static str, OrderedFloat<f64>>, Vec<usize>) {
        let mut graph = UndirectedWeightedGraph::new();
        let n0 = graph.base.add_node("A");
        let n1 = graph.base.add_node("B");
        let n2 = graph.base.add_node("C");
        let n3 = graph.base.add_node("D");

        graph.add_edge(n0, n1, OrderedFloat(1.0));
        graph.add_edge(n1, n2, OrderedFloat(2.0));
        graph.add_edge(n0, n2, OrderedFloat(4.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));

        (graph, vec![n0, n1, n2, n3])
    }

    #[derive(Default)]
struct Point {
        x: f64,
        y: f64,
    }

    fn setup_graph_point() -> (UndirectedWeightedGraph<Point, OrderedFloat<f64>>, Vec<usize>) {
        let mut graph = UndirectedWeightedGraph::new();
        let n0 = graph.base.add_node(Point { x: 0.0, y: 0.0 });
        let n1 = graph.base.add_node(Point { x: 1.0, y: 1.0 });
        let n2 = graph.base.add_node(Point { x: 2.0, y: 2.0 });
        let n3 = graph.base.add_node(Point { x: 3.0, y: 3.0 });

        graph.add_edge(n0, n1, 1.5.into());
        graph.add_edge(n1, n2, 2.5.into());
        graph.add_edge(n0, n2, 5.0.into());
        graph.add_edge(n2, n3, 1.0.into());

        (graph, vec![n0, n1, n2, n3])
    }

    #[test]
    fn test_a_star_int() {
        let (graph, nodes) = setup_graph_int();

        let heuristic = |_a: &usize, _b: &usize| -> i32 { 0 }; // Heurística trivial
        let (path, cost) = a_star_algorithm(&graph, nodes[0], nodes[3], heuristic).unwrap();

        // Camino esperado: A -> B -> C -> D
        assert_eq!(path, vec![nodes[0], nodes[1], nodes[2], nodes[3]]);
        assert_eq!(cost, 1 + 2 + 1); // 4
    }

    #[test]
    fn test_a_star_float() {
        let (graph, nodes) = setup_graph_float();

        let heuristic = |_a: &usize, _b: &usize| -> OrderedFloat<f64> { OrderedFloat(0.0) };
        let (path, cost) = a_star_algorithm(&graph, nodes[0], nodes[3], heuristic).unwrap();

        assert_eq!(path, vec![nodes[0], nodes[1], nodes[2], nodes[3]]);
        assert_eq!(cost, OrderedFloat(4.0)); // 1 + 2 + 1
    }

    fn euclidean_heuristic(
        graph: &UndirectedWeightedGraph<Point, OrderedFloat<f64>>,
        a_id: &usize,
        b_id: &usize,
    ) -> OrderedFloat<f64> {
        let a = graph.node_data(*a_id).unwrap();
        let b = graph.node_data(*b_id).unwrap();
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        OrderedFloat((dx * dx + dy * dy).sqrt())
    }


    #[test]
    fn test_a_star_point() {
        let (graph, nodes) = setup_graph_point();
        let heuristic = |a: &usize, b: &usize| euclidean_heuristic(&graph, a, b);
        let (path, cost) = a_star_algorithm(&graph, nodes[0], nodes[3], heuristic).unwrap();
        assert_eq!(path, vec![nodes[0], nodes[1], nodes[2], nodes[3]]);
        assert_eq!(cost, OrderedFloat(5.0));
    }

    #[test]
    fn test_a_star_traversal_points() {
        use ordered_float::OrderedFloat;

        let (graph, nodes) = setup_graph_point();

        let heuristic = |a: &usize, b: &usize| euclidean_heuristic(&graph, a, b);

        // Ejecutamos A*
        let result = a_star_traversal(&graph, nodes[0], nodes[3], heuristic);
        assert!(result.is_some(), "A* no encontró camino");

        let (path, cost, traversal_order) = result.unwrap();

        // Verificamos path esperado
        assert_eq!(path, vec![nodes[0], nodes[1], nodes[2], nodes[3]]);

        // Verificamos costo acumulado
        assert_eq!(cost, OrderedFloat(5.0)); // 1.5 + 2.5 + 1.0

        // Verificamos que el primer nodo del recorrido es el start
        assert_eq!(traversal_order.first(), Some(&nodes[0]));

        // Verificamos que el último nodo del recorrido contiene al goal (puede repetirse antes si hay mejores caminos)
        assert!(traversal_order.contains(&nodes[3]));
    }

}
