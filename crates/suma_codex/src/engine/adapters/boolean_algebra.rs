use crate::ast::CodexResult;
use crate::domains::boolean_algebra::ast::{BoolExpr, BooleanModel};
use crate::domains::queries::ast::QueryBlock;
use crate::engine::executors::DomainExecutor;
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

impl DomainExecutor for BooleanExecutor {
    fn execute(&mut self, result: &CodexResult, observer: &mut dyn FnMut(&str, CodexOutput)) -> bool {
        match result {
            CodexResult::Boolean(model) => {
                let mut cb = |label: &str, out: CodexOutput| observer(label, out);
                if let Err(e) = self.execute(model, &mut cb) {
                    observer("Error", CodexOutput::Error(format!("{}", e)));
                }
                true
            }
            _ => false,
        }
    }

    fn try_execute_query(&mut self, _query: &QueryBlock, _observer: &mut dyn FnMut(&str, CodexOutput)) -> bool {
        false // Boolean domain doesn't support cross-domain queries yet
    }
}

fn count_nodes(expr: &BoolExpr) -> usize {
    match expr {
        BoolExpr::Literal(_) | BoolExpr::Variable(_) => 1,
        BoolExpr::Not(inner) => 1 + count_nodes(inner),
        BoolExpr::BinaryOp { lhs, rhs, .. } => 1 + count_nodes(lhs) + count_nodes(rhs),
    }
}
