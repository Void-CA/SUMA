pub mod adapters;
pub mod dispatcher;
pub mod executor;
pub mod executors;

// Reexportamos para que el usuario pueda usar engine::CodexEngine
// en lugar de engine::dispatcher::CodexEngine
pub use dispatcher::CodexEngine;