use super::graph_base::GraphBase;

pub trait Directed: GraphBase {
    fn predecessors(&self, node: Self::NodeId) -> Vec<Self::NodeId>;
    fn successors(&self, node: Self::NodeId) -> Vec<Self::NodeId>;

    // Implementación por defecto para grado de entrada/salida
    fn in_degree(&self, node: Self::NodeId) -> usize {
        self.predecessors(node).len()
    }

    fn out_degree(&self, node: Self::NodeId) -> usize {
        self.successors(node).len()
    }
}