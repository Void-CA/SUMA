use crate::ast::CodexResult;
use crate::domains::queries::ast::QueryBlock;
use crate::outputs::CodexOutput;

/// Each domain adapter implements this trait.
/// The registry holds a collection of executors and dispatches to the right one.
pub trait DomainExecutor {
    /// Execute a block if it belongs to this domain.
    /// Returns `true` if the block was handled, `false` if this domain doesn't recognize it.
    fn execute(&mut self, result: &CodexResult, observer: &mut dyn FnMut(&str, CodexOutput)) -> bool;

    /// Handle a cross-domain query if this domain knows the target.
    /// Returns `true` if the query was handled.
    fn try_execute_query(
        &mut self,
        query: &QueryBlock,
        observer: &mut dyn FnMut(&str, CodexOutput),
    ) -> bool;
}

/// A registry of domain executors.
/// Iterates over registered executors and delegates to the first one that handles the block.
pub struct ExecutorRegistry {
    executors: Vec<Box<dyn DomainExecutor>>,
}

impl ExecutorRegistry {
    pub fn new() -> Self {
        Self {
            executors: Vec::new(),
        }
    }

    pub fn register<E: DomainExecutor + 'static>(&mut self, executor: E) {
        self.executors.push(Box::new(executor));
    }

    pub fn execute(
        &mut self,
        result: &CodexResult,
        observer: &mut dyn FnMut(&str, CodexOutput),
    ) -> bool {
        for exec in &mut self.executors {
            if exec.execute(result, observer) {
                return true;
            }
        }
        false
    }

    pub fn execute_query(
        &mut self,
        query: &QueryBlock,
        observer: &mut dyn FnMut(&str, CodexOutput),
    ) -> bool {
        for exec in &mut self.executors {
            if exec.try_execute_query(query, observer) {
                return true;
            }
        }
        false
    }
}
