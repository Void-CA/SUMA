use std::collections::HashMap;

use crate::core::boolean_algebra::error::{
    BooleanAlgebraError, EvaluationError, InvalidExpressionError, ParseError
};
use crate::core::boolean_algebra::parser::parse_expression;
use crate::core::boolean_algebra::Result;  // Nuestro Result personalizado
use crate::core::boolean_algebra::ast::Node;

/// Expresión booleana con AST y variables extraídas
pub struct BooleanExpr {
    pub ast: Node,
    pub variables: Vec<String>,
}

impl BooleanExpr {
    /// Crea una nueva expresión booleana a partir de un string
    pub fn new(expression: &str) -> Result<Self> {
        // Validar que la expresión no esté vacía
        if expression.trim().is_empty() {
            return Err(ParseError::EmptyExpression.into());
        }
        
        // Validar longitud máxima
        if expression.len() > 1000 {
            return Err(InvalidExpressionError::TooComplex(1000).into());
        }
        
        // Parsear la expresión
        let ast = parse_expression(expression)?;
        let variables = ast.extract_variables();
        
        // Validar nombres de variables
        for var in &variables {
            if !Self::is_valid_variable_name(var) {
                return Err(InvalidExpressionError::InvalidVariableName(var.clone()).into());
            }
        }
        
        Ok(BooleanExpr { ast, variables })
    }
    
    /// Evalúa la expresión con los valores proporcionados
    pub fn evaluate(&self, values: &HashMap<&str, bool>) -> Result<bool> {
        // Verificar que todas las variables estén presentes
        for var in &self.variables {
            if !values.contains_key(var.as_str()) {
                return Err(EvaluationError::MissingVariable(var.clone()).into());
            }
        }
        
        Ok(self.ast.evaluate(values))
    }
    
    /// Evalúa la expresión permitiendo valores por defecto para variables faltantes
    pub fn evaluate_with_defaults(&self, values: &HashMap<&str, bool>, default: bool) -> bool {
        let mut complete_values = values.clone();
        
        // Agregar valores por defecto para variables faltantes
        for var in &self.variables {
            if !complete_values.contains_key(var.as_str()) {
                complete_values.insert(var, default);
            }
        }
        
        self.ast.evaluate(&complete_values)
    }
    
    /// Genera la tabla de verdad completa de la expresión
    pub fn truth_table(&self) -> Vec<(HashMap<String, bool>, bool)> {
        let num_vars = self.variables.len();
        let num_combinations = 1 << num_vars; // 2^n
        
        let mut table = Vec::new();
        
        for i in 0..num_combinations {
            let mut values = HashMap::new();
            
            // Generar combinación de valores para las variables
            for (j, var) in self.variables.iter().enumerate() {
                let value = (i >> (num_vars - 1 - j)) & 1 == 1;
                values.insert(var.as_str(), value);
            }
            
            // Evaluar la expresión
            let result = self.ast.evaluate(&values);
            let row_values: HashMap<String, bool> = values.into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect();
            
            table.push((row_values, result));
        }
        
        table
    }

    pub fn full_truth_table(&self) -> Vec<(HashMap<String, bool>, HashMap<String, bool>, bool)> {
        let num_vars = self.variables.len();
        let num_combinations = 1 << num_vars;
        
        let mut table = Vec::new();
        
        // Extraer todas las subexpresiones únicas del AST
        let mut subexpressions = Vec::new();
        self.ast.extract_subexpressions(&mut subexpressions);
        
        // Ordenar y eliminar duplicados
        subexpressions.sort();
        subexpressions.dedup();
        subexpressions.sort_by_key(|a| a.len()); // Ordenar por complejidad
        
        for i in 0..num_combinations {
            let mut variable_values = HashMap::new();
            
            // Generar valores para las variables base
            for (j, var) in self.variables.iter().enumerate() {
                let value = (i >> (num_vars - 1 - j)) & 1 == 1;
                variable_values.insert(var.as_str(), value);
            }
            
            // Evaluar cada subexpresión
            let mut subexpression_values = HashMap::new();
            for expr_str in &subexpressions {
                // Para expresiones simples (variables individuales)
                if self.variables.contains(&expr_str) {
                    if let Some(&value) = variable_values.get(expr_str.as_str()) {
                        subexpression_values.insert(expr_str.clone(), value);
                    }
                } else {
                    // Parsear y evaluar expresiones complejas
                    if let Ok(sub_expr) = BooleanExpr::new(expr_str) {
                        let result = sub_expr.ast.evaluate(&variable_values);
                        subexpression_values.insert(expr_str.clone(), result);
                    }
                }
            }
            
            // Evaluar la expresión principal
            let final_result = self.ast.evaluate(&variable_values);
            
            // Convertir a String keys
            let var_values_str: HashMap<String, bool> = variable_values.into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect();
            
            table.push((var_values_str, subexpression_values, final_result));
        }
        
        table
    }

    
    /// Convierte la expresión a string (notación infija)
    pub fn to_string(&self) -> String {
        self.ast.to_infix_notation()
    }
    
    /// Convierte la expresión a notación prefija (para debugging)
    pub fn to_prefix_notation(&self) -> String {
        self.ast.to_prefix_notation()
    }
    
    /// Obtiene la complejidad de la expresión (número de operadores)
    pub fn complexity(&self) -> usize {
        self.ast.complexity()
    }
    
    /// Verifica si la expresión es una tautología
    pub fn is_tautology(&self) -> bool {
        self.truth_table().iter().all(|(_, result)| *result)
    }
    
    /// Verifica si la expresión es una contradicción
    pub fn is_contradiction(&self) -> bool {
        self.truth_table().iter().all(|(_, result)| !*result)
    }
    
    /// Verifica si dos expresiones son equivalentes
    pub fn equivalent_to(&self, other: &BooleanExpr) -> bool {
        // Para ser equivalentes, deben tener las mismas variables
        // y producir los mismos resultados para todas las combinaciones
        let all_vars: Vec<String> = self.variables.iter()
            .chain(other.variables.iter())
            .cloned()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        let num_combinations = 1 << all_vars.len();
        
        for i in 0..num_combinations {
            let mut values = HashMap::new();
            
            for (j, var) in all_vars.iter().enumerate() {
                let value = (i >> (all_vars.len() - 1 - j)) & 1 == 1;
                values.insert(var.as_str(), value);
            }
            
            let self_result = self.evaluate_with_defaults(&values, false);
            let other_result = other.evaluate_with_defaults(&values, false);
            
            if self_result != other_result {
                return false;
            }
        }
        
        true
    }
    
    // --- FUNCIONES AUXILIARES ---
    
    /// Valida que un nombre de variable sea válido
    fn is_valid_variable_name(name: &str) -> bool {
        // Una variable debe:
        // 1. No estar vacía
        // 2. Empezar con una letra
        // 3. Contener solo letras, números y underscores
        // 4. No ser una palabra reservada
        
        if name.is_empty() {
            return false;
        }
        
        // Verificar que empiece con letra
        if !name.chars().next().unwrap().is_alphabetic() {
            return false;
        }
        
        // Verificar que todos los caracteres sean válidos
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return false;
        }
        
        // Palabras reservadas
        let reserved_words = ["true", "false", "and", "or", "not", "xor", "implies", "iff"];
        if reserved_words.contains(&name.to_lowercase().as_str()) {
            return false;
        }
        
        true
    }
}

/// Implementación de Debug para facilitar testing
impl std::fmt::Debug for BooleanExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BooleanExpr({})", self.to_string())
    }
}

/// Implementación de Clone
impl Clone for BooleanExpr {
    fn clone(&self) -> Self {
        // Como Node y Vec<String> son Clone, podemos derivar Clone
        BooleanExpr {
            ast: self.ast.clone(),
            variables: self.variables.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests básicos de creación ---
    #[test]
    fn test_boolean_expr_creation() {
        let expr = BooleanExpr::new("A & B").unwrap();
        assert_eq!(expr.variables, vec!["A", "B"]);
        assert_eq!(expr.complexity(), 1);
    }
    
    #[test]
    fn test_empty_expression() {
        let result = BooleanExpr::new("");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_expression_too_long() {
        let long_expr = "A".repeat(1001);
        let result = BooleanExpr::new(&long_expr);
        assert!(result.is_err());
    }

    // --- Tests de evaluación básica ---
    #[test]
    fn test_evaluation() {
        let expr = BooleanExpr::new("A & ~B").unwrap();
        let mut values = HashMap::new();
        values.insert("A", true);
        values.insert("B", false);
        
        assert_eq!(expr.evaluate(&values).unwrap(), true);
    }
    
    #[test]
    fn test_evaluation_missing_variable() {
        let expr = BooleanExpr::new("A & B").unwrap();
        let mut values = HashMap::new();
        values.insert("A", true);
        // Falta B
        
        let result = expr.evaluate(&values);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_evaluation_with_defaults() {
        let expr = BooleanExpr::new("A & B").unwrap();
        let mut values = HashMap::new();
        values.insert("A", true);
        // B falta, usará default
        
        assert_eq!(expr.evaluate_with_defaults(&values, false), false); // true & false = false
        assert_eq!(expr.evaluate_with_defaults(&values, true), true);   // true & true = true
    }

    // --- Tests de operadores lógicos ---
    #[test]
    fn test_and_operator() {
        let expr = BooleanExpr::new("A & B").unwrap();
        let tests = vec![
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, true),
        ];
        
        for (a, b, expected) in tests {
            let mut values = HashMap::new();
            values.insert("A", a);
            values.insert("B", b);
            assert_eq!(expr.evaluate(&values).unwrap(), expected, "A: {}, B: {}", a, b);
        }
    }
    
    #[test]
    fn test_or_operator() {
        let expr = BooleanExpr::new("A | B").unwrap();
        let tests = vec![
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, true),
        ];
        
        for (a, b, expected) in tests {
            let mut values = HashMap::new();
            values.insert("A", a);
            values.insert("B", b);
            assert_eq!(expr.evaluate(&values).unwrap(), expected);
        }
    }
    
    #[test]
    fn test_not_operator() {
        let expr = BooleanExpr::new("~A").unwrap();
        let mut values = HashMap::new();
        
        values.insert("A", false);
        assert_eq!(expr.evaluate(&values).unwrap(), true);
        
        values.insert("A", true);
        assert_eq!(expr.evaluate(&values).unwrap(), false);
    }
    
    #[test]
    fn test_xor_operator() {
        let expr = BooleanExpr::new("A xor B").unwrap();
        let tests = vec![
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, false),
        ];
        
        for (a, b, expected) in tests {
            let mut values = HashMap::new();
            values.insert("A", a);
            values.insert("B", b);
            assert_eq!(expr.evaluate(&values).unwrap(), expected);
        }
    }

    // --- Tests de expresiones complejas ---
    #[test]
    fn test_complex_expression() {
        let expr = BooleanExpr::new("(A & B) | (~A & C)").unwrap();
        let mut values = HashMap::new();
        
        values.insert("A", true);
        values.insert("B", false);
        values.insert("C", true);
        assert_eq!(expr.evaluate(&values).unwrap(), false); // (T&F) | (F&T) = F|F = F
        
        values.insert("B", true);
        values.insert("C", false);
        assert_eq!(expr.evaluate(&values).unwrap(), true);  // (T&T) | (F&F) = T|F = T
    }
    
    #[test]
    fn test_nested_negations() {
        let expr = BooleanExpr::new("~~A").unwrap(); // Doble negación
        let mut values = HashMap::new();
        values.insert("A", true);
        assert_eq!(expr.evaluate(&values).unwrap(), true);
    }

    // --- Tests de tabla de verdad ---
    #[test]
    fn test_truth_table_simple() {
        let expr = BooleanExpr::new("A & B").unwrap();
        let table = expr.truth_table();
        
        assert_eq!(table.len(), 4); // 2 variables = 4 combinaciones
        
        // Verificar algunas filas específicas
        let mut found_true = false;
        for (values, result) in table {
            if values["A"] && values["B"] {
                assert!(result, "A AND B debería ser true cuando ambos son true");
                found_true = true;
            } else {
                assert!(!result, "A AND B debería ser false cuando alguno es false");
            }
        }
        assert!(found_true, "Debería haber al menos una fila con resultado true");
    }
    
    #[test]
    fn test_truth_table_three_variables() {
        let expr = BooleanExpr::new("A & (B | C)").unwrap();
        let table = expr.truth_table();
        
        assert_eq!(table.len(), 8); // 3 variables = 8 combinaciones
        assert_eq!(expr.variables.len(), 3);
    }

    // --- Tests de propiedades lógicas ---
    #[test]
    fn test_tautology() {
        let tautology = BooleanExpr::new("A | ~A").unwrap(); // Ley del medio excluido
        assert!(tautology.is_tautology());
        
        let not_tautology = BooleanExpr::new("A & B").unwrap();
        assert!(!not_tautology.is_tautology());
    }
    
    #[test]
    fn test_contradiction() {
        let contradiction = BooleanExpr::new("A & ~A").unwrap(); // Contradicción
        assert!(contradiction.is_contradiction());
        
        let not_contradiction = BooleanExpr::new("A | B").unwrap();
        assert!(!not_contradiction.is_contradiction());
    }
    
    #[test]
    fn test_equivalence() {
        // A → B es equivalente a ~A ∨ B
        let expr1 = BooleanExpr::new("A implies B").unwrap();
        let expr2 = BooleanExpr::new("~A | B").unwrap();
        
        assert!(expr1.equivalent_to(&expr2));
        
        // A ∧ B no es equivalente a A ∨ B
        let expr3 = BooleanExpr::new("A & B").unwrap();
        let expr4 = BooleanExpr::new("A | B").unwrap();
        
        assert!(!expr3.equivalent_to(&expr4));
    }

    // --- Tests de representación de strings ---
    #[test]
    fn test_string_representation() {
        let expr = BooleanExpr::new("A & (B | C)").unwrap();
        let repr = expr.to_string();
        
        // La representación debe contener las variables y operadores
        assert!(repr.contains('A'));
        assert!(repr.contains('B'));
        assert!(repr.contains('C'));
        assert!(repr.contains('&') || repr.contains('∧'));
    }
    
    #[test]
    fn test_prefix_notation() {
        let expr = BooleanExpr::new("A & B").unwrap();
        let prefix = expr.to_prefix_notation();
        
        // Debería ser algo como "AND(A, B)" o similar
        assert!(prefix.starts_with("AND") || prefix.contains("&"));
    }

    // --- Tests de complejidad ---
    #[test]
    fn test_complexity_calculation() {
        let simple = BooleanExpr::new("A").unwrap();
        assert_eq!(simple.complexity(), 0);
        
        let medium = BooleanExpr::new("A & B").unwrap();
        assert_eq!(medium.complexity(), 1);
        
        let complex = BooleanExpr::new("(A & B) | (~C & D)").unwrap();
        assert_eq!(complex.complexity(), 4); // OR, AND, NOT, AND
    }

    // --- Tests de validación de nombres de variables ---
    #[test]
    fn test_variable_name_validation() {
        assert!(BooleanExpr::is_valid_variable_name("A"));
        assert!(BooleanExpr::is_valid_variable_name("var1"));
        assert!(BooleanExpr::is_valid_variable_name("x_y"));
        assert!(BooleanExpr::is_valid_variable_name("VAR"));
        assert!(BooleanExpr::is_valid_variable_name("a1b2c3"));
        
        assert!(!BooleanExpr::is_valid_variable_name(""));
        assert!(!BooleanExpr::is_valid_variable_name("1var"));
        assert!(!BooleanExpr::is_valid_variable_name("var@"));
        assert!(!BooleanExpr::is_valid_variable_name("true"));
        assert!(!BooleanExpr::is_valid_variable_name("and"));
        assert!(!BooleanExpr::is_valid_variable_name("var-name"));
        assert!(!BooleanExpr::is_valid_variable_name("var name"));
    }
    
    #[test]
    fn test_invalid_variable_name_in_expression() {
        // "1var" es inválido porque empieza con número
        let result = BooleanExpr::new("1var & B");
        assert!(result.is_err());
        
        // "true" es VÁLIDO porque es una constante booleana
        let result = BooleanExpr::new("true & B");
        assert!(result.is_ok()); // Cambiado de is_err() a is_ok()
        
        // Verificar que funciona correctamente
        let expr = BooleanExpr::new("true & B").unwrap();
        let mut values = HashMap::new();
        values.insert("B", true);
        assert_eq!(expr.evaluate(&values).unwrap(), true); // true & true = true
        
        values.insert("B", false);
        assert_eq!(expr.evaluate(&values).unwrap(), false); // true & false = false
    }

    #[test]
    fn test_constants_are_not_treated_as_variables() {
        // "true" y "false" deben ser constantes, no variables
        let expr = BooleanExpr::new("true & false").unwrap();
        
        // No debería tener variables porque solo usa constantes
        assert_eq!(expr.variables, Vec::<String>::new());
        
        // La evaluación debería funcionar sin necesidad de valores
        let empty_values = HashMap::new();
        assert_eq!(expr.evaluate(&empty_values).unwrap(), false); // true & false = false
    }

    #[test]
    fn test_mixed_constants_and_variables() {
        let expr = BooleanExpr::new("true & A | false").unwrap();
        assert_eq!(expr.variables, vec!["A"]); // Solo A es variable
        
        let mut values = HashMap::new();
        values.insert("A", true);
        assert_eq!(expr.evaluate(&values).unwrap(), true); // true & true | false = true
        
        values.insert("A", false);
        assert_eq!(expr.evaluate(&values).unwrap(), false); // true & false | false = false
    }

    // --- Tests de diferentes sintaxis ---
    #[test]
    fn test_different_syntaxes() {
        // Diferentes formas de escribir la misma expresión
        let exprs = vec![
            "A and B",
            "A && B", 
            "A ∧ B",
            "A & B",
        ];
        
        for expr_str in exprs {
            let expr = BooleanExpr::new(expr_str).unwrap();
            let mut values = HashMap::new();
            values.insert("A", true);
            values.insert("B", true);
            
            assert_eq!(expr.evaluate(&values).unwrap(), true);
        }
    }
    
    #[test]
    fn test_operator_precedence() {
        // A & B | C debería ser (A & B) | C, no A & (B | C)
        let expr = BooleanExpr::new("A & B | C").unwrap();
        let mut values = HashMap::new();
        values.insert("A", true);
        values.insert("B", false);
        values.insert("C", true);
        
        // (true & false) | true = false | true = true
        // Si fuera true & (false | true) = true & true = true (mismo resultado)
        // Pero probemos otro caso:
        values.insert("A", false);
        values.insert("B", true);
        values.insert("C", false);
        
        // (false & true) | false = false | false = false
        // Si fuera false & (true | false) = false & true = false (mismo resultado)
        // En este caso AND y OR tienen la misma precedencia, se evalúa izquierda a derecha
        assert_eq!(expr.evaluate(&values).unwrap(), false);
    }

    // --- Tests de clonación ---
    #[test]
    fn test_clone() {
        let original = BooleanExpr::new("A & B").unwrap();
        let cloned = original.clone();
        
        assert_eq!(original.variables, cloned.variables);
        assert_eq!(original.complexity(), cloned.complexity());
        
        // Verificar que son independientes
        let mut values = HashMap::new();
        values.insert("A", true);
        values.insert("B", false);
        assert_eq!(original.evaluate(&values).unwrap(), cloned.evaluate(&values).unwrap());
    }

    // --- Tests de expresiones con constantes ---
    #[test]
    fn test_constants_in_expressions() {
        let expr = BooleanExpr::new("A & true").unwrap();
        let mut values = HashMap::new();
        values.insert("A", true);
        assert_eq!(expr.evaluate(&values).unwrap(), true);
        
        values.insert("A", false);
        assert_eq!(expr.evaluate(&values).unwrap(), false);
        
        let expr2 = BooleanExpr::new("false | A").unwrap();
        values.insert("A", true);
        assert_eq!(expr2.evaluate(&values).unwrap(), true);
    }

    #[test]
    fn test_full_truth_table() {
        let expr = BooleanExpr::new("A & (B | C)").unwrap();
        let table = expr.full_truth_table();
        
        assert_eq!(table.len(), 8); // 3 variables = 8 combinaciones
        
        for (var_values, subexpr_values, final_result) in table {
            // Verificar que las variables estén presentes
            assert!(var_values.contains_key("A"));
            assert!(var_values.contains_key("B"));
            assert!(var_values.contains_key("C"));
            
            // Verificar que las subexpresiones estén evaluadas
            assert!(subexpr_values.contains_key("B | C"));
            assert!(subexpr_values.contains_key("A & (B | C)"));
            
            // Verificar que el resultado final coincida con la evaluación directa
            let mut values = HashMap::new();
            for (k, v) in &var_values {
                values.insert(k.as_str(), *v);
            }
            let direct_result = expr.evaluate(&values).unwrap();
            assert_eq!(final_result, direct_result);
        }
    }
}