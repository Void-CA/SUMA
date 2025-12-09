use std::hash::Hash;

pub trait GraphBase {
    type NodeId: Clone + Eq + Hash + Copy;
    type NodeData;
    type EdgeData;

    fn nodes(&self) -> Vec<Self::NodeId>;
    fn edges(&self) -> Vec<(Self::NodeId, Self::NodeId)>;
    fn node_data(&self, id: Self::NodeId) -> Option<&Self::NodeData>;
    fn edge_data(&self, from: Self::NodeId, to: Self::NodeId) -> Option<&Self::EdgeData>;

    // Métodos comunes con implementación por defecto
    fn node_count(&self) -> usize {
        self.nodes().len()
    }

    fn edge_count(&self) -> usize {
        self.edges().len()
    }

    fn has_node(&self, id: Self::NodeId) -> bool {
        self.node_data(id).is_some()
    }
    
    fn neighbors(&self, node: Self::NodeId) -> Vec<Self::NodeId>;
}