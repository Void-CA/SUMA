pub mod dispatcher;
pub mod executor;
mod adapters;

// Reexportamos para que el usuario pueda usar engine::CodexEngine
// en lugar de engine::dispatcher::CodexEngine
pub use dispatcher::CodexEngine;