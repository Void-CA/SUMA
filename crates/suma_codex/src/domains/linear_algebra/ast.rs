use serde::Serialize;

// 1. El Modelo Raíz (Equivalente a BooleanModel)
#[derive(Debug, Serialize, Clone)]
pub struct LinearAlgebraModel {
    pub name: Option<String>,
    // En AL, un "programa" es una lista de instrucciones secuenciales
    pub statements: Vec<LinAlgStmt>,
}

// 2. Sentencias (Statements)
#[derive(Debug, Serialize, Clone)]
pub enum LinAlgStmt {
    Assignment {
        target: String,
        value: LinAlgExpr,
    },
    Evaluation(LinAlgExpr),
}

// 3. Expresiones (Expressions)
#[derive(Debug, Serialize, Clone)]
pub enum LinAlgExpr {
    Number(f64),
    Variable(String),
    // Representación serializable de una matriz [ [1, 2], [3, 4] ]
    Matrix(Vec<Vec<LinAlgExpr>>),

    BinaryOp {
        op: String, // "+", "-", "*"
        lhs: Box<LinAlgExpr>, // Renombrado a lhs/rhs para consistencia con BoolExpr
        rhs: Box<LinAlgExpr>,
    },

    Call {
        function: String,
        args: Vec<LinAlgExpr>,
    },
}