use std::collections::HashSet;
use std::fmt;

/// Nodo del Árbol de Sintaxis Abstracta (AST) para expresiones booleanas
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// Variable booleana (ej: "A", "B", "x1")
    Variable(String),
    /// Operación AND (conjunción) entre dos expresiones
    And(Box<Node>, Box<Node>),
    /// Operación OR (disyunción) entre dos expresiones  
    Or(Box<Node>, Box<Node>),
    /// Operación NOT (negación) de una expresión
    Not(Box<Node>),
    /// Valor constante (true o false)
    Constant(bool),
    /// Operación XOR (o exclusivo) - útil para expansión
    Xor(Box<Node>, Box<Node>),
    /// Operación NAND (no y) - útil para expansión
    Nand(Box<Node>, Box<Node>),
    /// Operación NOR (no o) - útil para expansión
    Nor(Box<Node>, Box<Node>),
    /// Operación IMPLIES (implicación) A → B
    Implies(Box<Node>, Box<Node>),
    /// Operación IFF (doble implicación) A ↔ B
    Iff(Box<Node>, Box<Node>),
}

impl Node {
    /// Extrae todas las variables únicas de la expresión
    pub fn extract_variables(&self) -> Vec<String> {
        let mut variables = HashSet::new();
        self.extract_variables_recursive(&mut variables);
        
        let mut vars: Vec<String> = variables.into_iter().collect();
        vars.sort(); // Orden alfabético para consistencia
        vars
    }
    
    /// Recorrido recursivo para extraer variables
    fn extract_variables_recursive(&self, variables: &mut HashSet<String>) {
        match self {
            Node::Variable(name) => {
                variables.insert(name.clone());
            }
            Node::And(left, right) 
            | Node::Or(left, right) 
            | Node::Xor(left, right)
            | Node::Implies(left, right)
            | Node::Iff(left, right) => {
                left.extract_variables_recursive(variables);
                right.extract_variables_recursive(variables);
            }
            Node::Not(inner) => {
                inner.extract_variables_recursive(variables);
            }
            Node::Nand(left, right) => {
                left.extract_variables_recursive(variables);
                right.extract_variables_recursive(variables);
            }
            Node::Nor(left, right) => {
                left.extract_variables_recursive(variables);
                right.extract_variables_recursive(variables);
            }
            Node::Constant(_) => {} // Las constantes no tienen variables
        }
    }
    
    /// Evalúa la expresión con los valores dados
    pub fn evaluate(&self, values: &std::collections::HashMap<&str, bool>) -> bool {
        match self {
            Node::Variable(name) => {
                *values.get(name.as_str()).unwrap_or(&false)
            }
            Node::And(left, right) => {
                left.evaluate(values) && right.evaluate(values)
            }
            Node::Or(left, right) => {
                left.evaluate(values) || right.evaluate(values)
            }
            Node::Not(inner) => {
                !inner.evaluate(values)
            }
            Node::Constant(value) => *value,
            Node::Xor(left, right) => {
                left.evaluate(values) ^ right.evaluate(values)
            }
            Node::Implies(left, right) => {
                !left.evaluate(values) || right.evaluate(values)
            }
            Node::Iff(left, right) => {
                left.evaluate(values) == right.evaluate(values)
            }
            Node::Nand(left, right) => {
                !(left.evaluate(values) && right.evaluate(values))
            }
            Node::Nor(left, right) => {
                !(left.evaluate(values) || right.evaluate(values))
            }
        }
    }
    
    /// Convierte el AST a una representación de string (notación prefija)
    pub fn to_prefix_notation(&self) -> String {
        match self {
            Node::Variable(name) => name.clone(),
            Node::And(left, right) => {
                format!("AND({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
            Node::Or(left, right) => {
                format!("OR({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
            Node::Not(inner) => {
                format!("NOT({})", inner.to_prefix_notation())
            }
            Node::Constant(value) => value.to_string(),
            Node::Xor(left, right) => {
                format!("XOR({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
            Node::Implies(left, right) => {
                format!("IMPLIES({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
            Node::Iff(left, right) => {
                format!("IFF({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
            Node::Nand(left, right) => {
                format!("NAND({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
            Node::Nor(left, right) => {
                format!("NOR({}, {})", left.to_prefix_notation(), right.to_prefix_notation())
            }
        }
    }
    
    /// Convierte el AST a una representación infija (notación tradicional)
    pub fn to_infix_notation(&self) -> String {
        match self {
            Node::Variable(name) => name.clone(),
            Node::And(left, right) => {
                format!("({} ∧ {})", left.to_infix_notation(), right.to_infix_notation())
            }
            Node::Or(left, right) => {
                format!("({} ∨ {})", left.to_infix_notation(), right.to_infix_notation())
            }
            Node::Not(inner) => {
                format!("¬{}", inner.to_infix_notation())
            }
            Node::Constant(value) => value.to_string(),
            Node::Xor(left, right) => {
                format!("({} ⊕ {})", left.to_infix_notation(), right.to_infix_notation())
            }
            Node::Implies(left, right) => {
                format!("({} → {})", left.to_infix_notation(), right.to_infix_notation())
            }
            Node::Iff(left, right) => {
                format!("({} ↔ {})", left.to_infix_notation(), right.to_infix_notation())
            }
            Node::Nand(left, right) => {
                format!("¬({} ∧ {})", left.to_infix_notation(), right.to_infix_notation())
            }
            Node::Nor(left, right) => {
                format!("¬({} ∨ {})", left.to_infix_notation(), right.to_infix_notation())
            }
        }
    }
    
    /// Calcula la complejidad de la expresión (número de operadores)
    pub fn complexity(&self) -> usize {
        match self {
            Node::Variable(_) | Node::Constant(_) => 0,
            Node::Not(inner) => 1 + inner.complexity(),
            Node::And(left, right) 
            | Node::Or(left, right)
            | Node::Xor(left, right)
            | Node::Implies(left, right)
            | Node::Iff(left, right) => {
                1 + left.complexity() + right.complexity()
            }
            Node::Nand(left, right)
            | Node::Nor(left, right) => {
                1 + left.complexity() + right.complexity()
            }

        }
    }

    pub fn extract_subexpressions(&self, expressions: &mut Vec<String>) {
        match self {
            Node::Variable(_) | Node::Constant(_) => {
                // No hacer nada para variables y constantes
            }
            Node::Not(expr) => {
                let expr_str = expr.to_string();
                expressions.push(format!("NOT {}", expr_str));
                expressions.push(expr_str);
                expr.extract_subexpressions(expressions);
            }
            Node::And(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} AND {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
            Node::Or(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} OR {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
            Node::Xor(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} XOR {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
            Node::Implies(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} IMPLIES {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
            Node::Iff(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} IFF {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
            Node::Nand(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} NAND {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
            Node::Nor(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                expressions.push(format!("({} NOR {})", left_str, right_str));
                expressions.push(left_str.clone());
                expressions.push(right_str.clone());
                left.extract_subexpressions(expressions);
                right.extract_subexpressions(expressions);
            }
        }
    }

}

/// Implementación de Display para fácil debugging
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
    }
}

/// Métodos de conveniencia para crear nodos
impl Node {
    /// Crea un nodo variable
    pub fn var(name: &str) -> Self {
        Node::Variable(name.to_string())
    }
    
    /// Crea un nodo constante
    pub fn constant(value: bool) -> Self {
        Node::Constant(value)
    }
    
    /// Crea una operación AND entre dos nodos
    pub fn and(left: Node, right: Node) -> Self {
        Node::And(Box::new(left), Box::new(right))
    }
    
    /// Crea una operación OR entre dos nodos  
    pub fn or(left: Node, right: Node) -> Self {
        Node::Or(Box::new(left), Box::new(right))
    }
    
    /// Crea una operación NOT
    pub fn not(inner: Node) -> Self {
        Node::Not(Box::new(inner))
    }

    /// Crea una operación XOR entre dos nodos
    pub fn xor(left: Node, right: Node) -> Self {
        Node::Xor(Box::new(left), Box::new(right))
    }

    /// Crea una operación IMPLIES entre dos nodos
    pub fn implies(left: Node, right: Node) -> Self {
        Node::Implies(Box::new(left), Box::new(right))
    }

    /// Crea una operación IFF entre dos nodos
    pub fn iff(left: Node, right: Node) -> Self {
        Node::Iff(Box::new(left), Box::new(right))
    }

    /// Crea una operación NAND entre dos nodos
    pub fn nand(left: Node, right: Node) -> Self {
        Node::Nand(Box::new(left), Box::new(right))
    }

    /// Crea una operación NOR entre dos nodos
    pub fn nor(left: Node, right: Node) -> Self {
        Node::Nor(Box::new(left), Box::new(right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_extraction() {
        let expr = Node::and(
            Node::var("A"),
            Node::or(Node::var("B"), Node::var("C"))
        );
        
        let variables = expr.extract_variables();
        assert_eq!(variables, vec!["A", "B", "C"]);
    }
    
    #[test]
    fn test_evaluation() {
        let expr = Node::and(Node::var("A"), Node::not(Node::var("B")));
        let mut values = std::collections::HashMap::new();
        values.insert("A", true);
        values.insert("B", false);
        
        assert_eq!(expr.evaluate(&values), true);
    }
    
    #[test]
    fn test_complexity() {
        let simple = Node::var("A");
        let complex = Node::and(
            Node::var("A"), 
            Node::or(Node::var("B"), Node::not(Node::var("C")))
        );
        
        assert_eq!(simple.complexity(), 0);
        assert_eq!(complex.complexity(), 3); // AND, OR, NOT
    }
}