use super::graph_base::GraphBase;
use num_traits::{Num, Zero};

pub trait Weight: Copy + Ord + std::ops::Add<Output = Self> + num_traits::Zero+ Num {
    fn inf() -> Self;
}

// Implementaciones para enteros
impl Weight for i32 {
    fn inf() -> Self { i32::MAX }
}

// Implementaciones para floats
impl Weight for ordered_float::OrderedFloat<f32> {
    fn inf() -> Self { ordered_float::OrderedFloat(f32::INFINITY) }
}

impl Weight for ordered_float::OrderedFloat<f64> {
    fn inf() -> Self { ordered_float::OrderedFloat(f64::INFINITY) }
}



pub trait WeightedGraph : GraphBase
where
    Self::NodeId: Clone,
    Self::EdgeData: Weight,
{
    fn edge_weight(&self, from: Self::NodeId, to: Self::NodeId) -> Option<Self::EdgeData>;

    fn total_weight(&self) -> Self::EdgeData {
        self.edges()
            .iter()
            .filter_map(|(from, to)| self.edge_weight(from.clone(), to.clone()))
            .fold(Self::EdgeData::zero(), |acc, w| acc + w)
    }
}
