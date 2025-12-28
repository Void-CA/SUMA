// src/symbolics/context.rs
use std::collections::HashMap;

#[derive(Default)]
pub struct Context {
    values: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self { Self::default() }
    
    pub fn set(&mut self, name: &str, val: f64) {
        self.values.insert(name.to_string(), val);
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.values.get(name).copied()
    }
}