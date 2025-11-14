use std::collections::HashMap;
use crate::core::data_structures::graphs::{
    implementations::base_graph::BaseGraph,
    traits::{GraphBase, DirectedGraph}
};

pub struct DirectedSimpleGraph<T> {
    pub base: BaseGraph<T, ()>,
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl<T> DirectedSimpleGraph<T> {
    pub fn new() -> Self {
        Self {
            base: BaseGraph::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_directed_edge(&mut self, from: usize, to: usize) {
        self.base.add_edge(from, to, ());
        self.adjacency.entry(from).or_insert_with(Vec::new).push(to);
    }
}

impl<T> GraphBase for DirectedSimpleGraph<T> {
    type NodeId = usize;
    type NodeData = T;
    type EdgeData = ();

    fn nodes(&self) -> Vec<usize> {
        self.base.nodes.keys().cloned().collect()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.base.edges.keys().cloned().collect()
    }

    fn node_data(&self, id: usize) -> Option<&T> {
        self.base.nodes.get(&id)
    }

    fn edge_data(&self, from: usize, to: usize) -> Option<&()> {
        self.base.edges.get(&(from, to))
    }

    fn neighbors(&self, node: Self::NodeId) -> Vec<Self::NodeId> {
        self.successors(node)
    }
}

impl<T> DirectedGraph for DirectedSimpleGraph<T> {
    fn predecessors(&self, node: usize) -> Vec<usize> {
        self.adjacency
            .iter()
            .filter_map(|(&from, neighbors)| {
                if neighbors.contains(&node) {
                    Some(from)
                } else {
                    None
                }
            })
            .collect()
    }

    fn successors(&self, node: usize) -> Vec<usize> {
        self.adjacency.get(&node).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph: DirectedSimpleGraph<i32> = DirectedSimpleGraph::new();
        assert!(graph.base.nodes.is_empty());
        assert!(graph.base.edges.is_empty());
        assert!(graph.adjacency.is_empty());
        assert_eq!(graph.base.next_id, 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = DirectedSimpleGraph::new();
        let node_id = graph.base.add_node("test_data");

        assert_eq!(node_id, 0);
        assert_eq!(graph.base.nodes.len(), 1);
        assert_eq!(graph.base.next_id, 1);
        assert_eq!(graph.base.nodes.get(&0), Some(&"test_data"));
    }

    #[test]
    fn test_add_directed_edge() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node("node1");
        let node2 = graph.base.add_node("node2");

        graph.add_directed_edge(node1, node2);

        // Verificar que la arista existe en base.edges
        assert!(graph.base.edges.contains_key(&(node1, node2)));
        // Verificar que la adyacencia se actualizó correctamente
        assert_eq!(graph.adjacency.get(&node1), Some(&vec![node2]));
        // Verificar que node2 no tiene sucesores (grafo dirigido)
        assert_eq!(graph.adjacency.get(&node2), None);
    }

    #[test]
    fn test_nodes_method() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(10);
        let node2 = graph.base.add_node(20);

        let nodes = graph.nodes();

        assert_eq!(nodes.len(), 2);
        assert!(nodes.contains(&node1));
        assert!(nodes.contains(&node2));
    }

    #[test]
    fn test_edges_method() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);
        let node3 = graph.base.add_node(3);

        graph.add_directed_edge(node1, node2);
        graph.add_directed_edge(node2, node3);

        let edges = graph.edges();

        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(node1, node2)));
        assert!(edges.contains(&(node2, node3)));
    }

    #[test]
    fn test_node_data() {
        let mut graph = DirectedSimpleGraph::new();
        let node_id = graph.base.add_node("test_data");

        assert_eq!(graph.node_data(node_id), Some(&"test_data"));
        assert_eq!(graph.node_data(999), None); // Nodo inexistente
    }

    #[test]
    fn test_edge_data() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);

        graph.add_directed_edge(node1, node2);

        assert_eq!(graph.edge_data(node1, node2), Some(&()));
        assert_eq!(graph.edge_data(node2, node1), None); // Arista inversa no existe
        assert_eq!(graph.edge_data(0, 999), None); // Arista inexistente
    }

    #[test]
    fn test_successors() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);
        let node3 = graph.base.add_node(3);

        graph.add_directed_edge(node1, node2);
        graph.add_directed_edge(node1, node3);
        graph.add_directed_edge(node2, node3);

        assert_eq!(graph.successors(node1), vec![node2, node3]);
        assert_eq!(graph.successors(node2), vec![node3]);
        assert_eq!(graph.successors(node3), Vec::<usize>::new()); // Sin sucesores
        assert_eq!(graph.successors(999), Vec::<usize>::new()); // Nodo inexistente
    }

    #[test]
    fn test_predecessors() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);
        let node3 = graph.base.add_node(3);

        graph.add_directed_edge(node1, node3);
        graph.add_directed_edge(node2, node3);
        graph.add_directed_edge(node3, node1); // Ciclo

        assert_eq!(graph.predecessors(node3), vec![node1, node2]);
        assert_eq!(graph.predecessors(node1), vec![node3]);
        assert_eq!(graph.predecessors(node2), Vec::<usize>::new()); // Sin predecesores
        assert_eq!(graph.predecessors(999), Vec::<usize>::new()); // Nodo inexistente
    }

    #[test]
    fn test_multiple_edges_from_same_node() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(1);
        let node2 = graph.base.add_node(2);
        let node3 = graph.base.add_node(3);

        graph.add_directed_edge(node1, node2);
        graph.add_directed_edge(node1, node3);

        assert_eq!(graph.successors(node1), vec![node2, node3]);
        assert_eq!(graph.adjacency.get(&node1), Some(&vec![node2, node3]));
    }

    #[test]
    fn test_self_loop() {
        let mut graph = DirectedSimpleGraph::new();
        let node1 = graph.base.add_node(1);

        graph.add_directed_edge(node1, node1); // Self-loop

        assert_eq!(graph.successors(node1), vec![node1]);
        assert_eq!(graph.predecessors(node1), vec![node1]);
        assert!(graph.base.edges.contains_key(&(node1, node1)));
    }

    #[test]
    fn test_complex_graph_structure() {
        let mut graph = DirectedSimpleGraph::new();
        let nodes: Vec<usize> = (0..5).map(|i| graph.base.add_node(i)).collect();

        // Crear una estructura más compleja
        graph.add_directed_edge(nodes[0], nodes[1]);
        graph.add_directed_edge(nodes[0], nodes[2]);
        graph.add_directed_edge(nodes[1], nodes[3]);
        graph.add_directed_edge(nodes[2], nodes[3]);
        graph.add_directed_edge(nodes[3], nodes[4]);

        // Verificar sucesores
        assert_eq!(graph.successors(nodes[0]), vec![nodes[1], nodes[2]]);
        assert_eq!(graph.successors(nodes[3]), vec![nodes[4]]);

        // Verificar predecesores
        assert_eq!(graph.predecessors(nodes[3]), vec![nodes[1], nodes[2]]);
        assert_eq!(graph.predecessors(nodes[4]), vec![nodes[3]]);

        // Verificar nodos sin conexiones
        assert_eq!(graph.successors(nodes[4]), Vec::<usize>::new());
        assert_eq!(graph.predecessors(nodes[0]), Vec::<usize>::new());
    }

    #[test]
    fn test_edge_cases() {
        let mut graph: DirectedSimpleGraph<&str> = DirectedSimpleGraph::new();

        // Agregar nodos y luego aristas válidas
        let existing_node = graph.base.add_node("exists");
        let another_node = graph.base.add_node("another");

        graph.add_directed_edge(existing_node, another_node);

        assert_eq!(graph.edges().len(), 1);
        assert_eq!(graph.nodes().len(), 2);
    }
}