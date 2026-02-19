use super::graph_base::GraphBase;
use num_traits::{Num, Zero};
use ordered_float::OrderedFloat;

pub trait Weight: Copy + Ord + std::ops::Add<Output = Self> + num_traits::Zero+ Num {
    fn inf() -> Self;
}

// Implementaciones para enteros
impl Weight for i32 {
    fn inf() -> Self { i32::MAX }
}

// Implementaciones para floats
impl Weight for OrderedFloat<f32> {
    fn inf() -> Self { OrderedFloat(f32::INFINITY) }
}

impl Weight for OrderedFloat<f64> {
    fn inf() -> Self { OrderedFloat(f64::INFINITY) }
}

pub trait IntoWeight<E> {
    fn into_weight(self) -> E;
}

impl IntoWeight<OrderedFloat<f64>> for f64 {
    fn into_weight(self) -> OrderedFloat<f64> { OrderedFloat::from(self) }
}

impl IntoWeight<OrderedFloat<f32>> for f32 {
    fn into_weight(self) -> OrderedFloat<f32> { OrderedFloat::from(self) }
    
}
impl<E: Copy> IntoWeight<E> for E {
    fn into_weight(self) -> E { self }
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
