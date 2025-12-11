pub mod dispatcher;
pub mod executor;

// Re-exportamos para que el usuario pueda usar engine::CodexEngine
// en lugar de engine::dispatcher::CodexEngine
pub use dispatcher::CodexEngine;