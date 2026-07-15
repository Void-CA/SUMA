use crate::domains::boolean_algebra::ast::{BoolExpr, BooleanModel};
use crate::error::CodexError;
use crate::outputs::CodexOutput;

pub struct BooleanExecutor;

impl BooleanExecutor {
    pub fn new(_verbose: bool) -> Self {
        BooleanExecutor
    }

    pub fn execute<F>(&mut self, model: &BooleanModel, observer: &mut F) -> Result<(), CodexError>
    where
        F: FnMut(&str, CodexOutput),
    {
        let name = model.name.as_deref().unwrap_or("unnamed");
        observer("System", CodexOutput::Message(
            format!("Boolean expression '{}' registered with {} sub-expressions", name, count_nodes(&model.root))
        ));
        Ok(())
    }
}

fn count_nodes(expr: &BoolExpr) -> usize {
    match expr {
        BoolExpr::Literal(_) | BoolExpr::Variable(_) => 1,
        BoolExpr::Not(inner) => 1 + count_nodes(inner),
        BoolExpr::BinaryOp { lhs, rhs, .. } => 1 + count_nodes(lhs) + count_nodes(rhs),
    }
}
