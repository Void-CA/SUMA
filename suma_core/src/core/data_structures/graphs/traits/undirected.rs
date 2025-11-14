use super::graph_base::GraphBase;

pub trait UndirectedGraph: GraphBase {

    // Implementación por defecto para grado
    fn degree(&self, node: Self::NodeId) -> usize {
        self.neighbors(node).len()
    }
}