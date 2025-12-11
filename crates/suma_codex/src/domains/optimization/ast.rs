use serde::Serialize;

// --- Expresiones Matemáticas ---

#[derive(Debug, Serialize, Clone)]
pub enum Op {
    Add, Sub, Mul, Div
}

#[derive(Debug, Serialize, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

// --- Estructuras del Modelo ---

#[derive(Debug, Serialize, Clone)]
pub enum OptType {
    Maximize,
    Minimize,
}

#[derive(Debug, Serialize, Clone)]
pub struct Constraint {
    pub lhs: Expr,
    pub op: String,
    pub rhs: Expr,
}

#[derive(Debug, Serialize, Clone)]
pub struct OptimizationModel {
    pub name: Option<String>,
    pub obj_type: OptType,
    pub obj_expr: Expr,
    pub constraints: Vec<Constraint>,
}