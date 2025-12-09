use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum BoolOp {
    And, Or
}

#[derive(Debug, Serialize, Clone)]
pub enum BoolExpr {
    Literal(bool),
    Variable(String),
    Not(Box<BoolExpr>),
    BinaryOp {
        op: BoolOp,
        lhs: Box<BoolExpr>,
        rhs: Box<BoolExpr>,
    },
}

// El resultado final de este dominio es simplemente una expresión raíz
#[derive(Debug, Serialize, Clone)]
pub struct BooleanModel {
    pub root: BoolExpr,
}