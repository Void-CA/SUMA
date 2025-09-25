use pyo3::prelude::*;
use pyo3::types::{PyDict};
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

use crate::core::boolean_algebra::BooleanExpr;

#[pyclass]
pub struct TruthTable {
    #[pyo3(get)]
    pub variables: Vec<String>,
    #[pyo3(get)]
    pub combinations: Vec<Vec<bool>>,
    #[pyo3(get)]
    pub results: Vec<bool>,
}

#[pymethods]
impl TruthTable {
    pub fn to_pretty_string(&self) -> String {
        if self.variables.is_empty() {
            return "Empty TruthTable".to_string();
        }
        
        let mut output = String::new();
        
        // Encabezado con nombres de variables
        for var in &self.variables {
            output.push_str(&format!("│ {:^8} ", var));
        }
        // Header for the result column
        output.push_str(&format!("│ {:^8} │\n", "Result"));
        
        // Línea separadora
        for _ in &self.variables {
            output.push_str("├──────────");
        }
        output.push_str("┼──────────┤\n");
        
        // Filas de datos
        for (i, row) in self.combinations.iter().enumerate() {
            for &val in row {
                output.push_str(&format!("│ {:^8} ", val));
            }
            output.push_str(&format!("│ {:^8} │\n", self.results[i]));
        }
        
        // Línea final
        for _ in &self.variables {
            output.push_str("└──────────");
        }
        output.push_str("┴──────────┘\n");
        
        output
    }
    fn to_polars(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let polars = py.import("polars")?;
        
        // Intentar diferentes métodos de construcción
        let data_dict = PyDict::new(py);
        
        // Preparar los datos
        for (i, var) in self.variables.iter().enumerate() {
            let values: Vec<bool> = self.combinations.iter()
                .map(|row| row[i])
                .collect();
            data_dict.set_item(var, values)?;
        }
        data_dict.set_item("result", &self.results)?;
        
        // Método 1: polars.from_dict (recomendado)
        if let Ok(from_dict) = polars.getattr("from_dict") {
            return Ok(from_dict.call1((data_dict,))?.into());
        }
        
        // Método 2: polars.DataFrame(dict)
        let dataframe = polars.getattr("DataFrame")?;
        Ok(dataframe.call1((data_dict,))?.into())
    }
    
    // Versión que retorna LazyFrame
    fn to_lazyframe(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let df = self.to_polars(py)?;
        Python::attach(|py| {
            let df_bound = df.bind(py);
            let lazyframe = df_bound.call_method0("lazy")?;
            Ok(lazyframe.into())
        })
    }
    
    fn to_dict(&self) -> PyResult<Py<PyDict>> {
        Python::attach(|py| {
            let result = PyDict::new(py);
            
            // Diccionario con listas (formato columnar)
            let column_dict = PyDict::new(py);
            
            for (i, var) in self.variables.iter().enumerate() {
                let values: Vec<bool> = self.combinations.iter()
                    .map(|row| row[i])
                    .collect();
                column_dict.set_item(var, values)?;
            }
            column_dict.set_item("result", &self.results)?;
            
            result.set_item("columns", column_dict)?;
            result.set_item("variables", &self.variables)?;
            result.set_item("row_count", self.combinations.len())?;
            
            Ok(result.into())
        })
    }
    
    fn to_list(&self) -> Vec<HashMap<String, bool>> {
        self.combinations.iter().enumerate().map(|(i, row)| {
            let mut map = HashMap::new();
            for (j, var) in self.variables.iter().enumerate() {
                map.insert(var.clone(), row[j]);
            }
            map.insert("result".to_string(), self.results[i]);
            map
        }).collect()
    }
    // Filtrar solo las combinaciones que son TRUE
    fn filter_true(&self) -> Self {
        let mut true_combinations = Vec::new();
        let mut true_results = Vec::new();
        
        for (i, result) in self.results.iter().enumerate() {
            if *result {
                true_combinations.push(self.combinations[i].clone());
                true_results.push(true);
            }
        }
        
        TruthTable {
            variables: self.variables.clone(),
            combinations: true_combinations,
            results: true_results,
        }
    }
    
    fn filter_false(&self) -> Self {
        let mut false_combinations = Vec::new();
        let mut false_results = Vec::new();
        
        for (i, result) in self.results.iter().enumerate() {
            if !*result {
                false_combinations.push(self.combinations[i].clone());
                false_results.push(false);
            }
        }
        
        TruthTable {
            variables: self.variables.clone(),
            combinations: false_combinations,
            results: false_results,
        }
    }

    fn __str__(&self) -> String {
        self.to_pretty_string()
    }
    
    // Obtener estadísticas básicas
    fn summary(&self) -> PyResult<Py<PyDict>> {
        Python::attach(|py| {
            let summary = PyDict::new(py);
            
            let true_count = self.results.iter().filter(|&&r| r).count();
            let false_count = self.results.len() - true_count;
            let total = self.results.len();
            
            summary.set_item("total_combinations", total)?;
            summary.set_item("true_count", true_count)?;
            summary.set_item("false_count", false_count)?;
            summary.set_item("true_percentage", (true_count as f64 / total as f64) * 100.0)?;
            
            Ok(summary.into())
        })
    }
}

#[pyclass]
pub struct FullTruthTable {
    #[pyo3(get)]
    pub base_variables: Vec<String>,
    #[pyo3(get)]
    pub step_labels: Vec<String>,
    #[pyo3(get)]
    pub step_results: Vec<Vec<bool>>,
}

#[pymethods]
impl FullTruthTable {
    #[new]
    fn new(base_variables: Vec<String>, step_labels: Vec<String>, step_results: Vec<Vec<bool>>) -> Self {
        FullTruthTable {
            base_variables,
            step_labels,
            step_results,
        }
    }
    
    #[getter]
    fn base_combinations(&self) -> Vec<Vec<bool>> {
        let num_base_vars = self.base_variables.len();
        self.step_results.iter()
            .map(|row| row[..num_base_vars].to_vec())
            .collect()
    }
    
    #[getter]
    fn final_results(&self) -> Vec<bool> {
        self.step_results.iter()
            .map(|row| *row.last().unwrap_or(&false))
            .collect()
    }
    
    fn num_rows(&self) -> usize {
        self.step_results.len()
    }
    
    fn num_steps(&self) -> usize {
        self.step_labels.len()
    }
    
    fn to_polars(&self, py: Python<'_>) -> PyResult<PyObject> {
        let polars = py.import("polars")?;
        let from_dict = polars.getattr("from_dict")?;
        
        let data_dict = PyDict::new(py);
        
        // Agregar cada columna
        for (i, label) in self.step_labels.iter().enumerate() {
            let values: Vec<bool> = self.step_results.iter()
                .map(|row| row[i])
                .collect();
            data_dict.set_item(label, values)?;
        }
        
        let df = from_dict.call1((data_dict,))?;
        Ok(df.into())
    }
    
    fn to_pretty_string(&self) -> String {
        if self.step_labels.is_empty() {
            return "Empty FullTruthTable".to_string();
        }
        
        let mut output = String::new();
        
        // Encabezado
        for label in &self.step_labels {
            output.push_str(&format!("│ {:^12} ", label));
        }
        output.push_str("│\n");
        
        // Separador
        for _ in &self.step_labels {
            output.push_str("├──────────────");
        }
        output.push_str("┤\n");
        
        // Filas de datos
        for row in &self.step_results {
            for &val in row {
                output.push_str(&format!("│ {:^12} ", val));
            }
            output.push_str("│\n");
        }
        
        // Pie de tabla
        for _ in &self.step_labels {
            output.push_str("└──────────────");
        }
        output.push_str("┘\n");
        
        output
    }
    
    fn __str__(&self) -> String {
        self.to_pretty_string()
    }
    
    fn __repr__(&self) -> String {
        format!("FullTruthTable(base_variables: {:?}, steps: {}, rows: {})", 
                self.base_variables, self.num_steps(), self.num_rows())
    }
    
    fn __len__(&self) -> usize {
        self.num_rows()
    }
    
    // Método para acceder a una fila específica
    fn get_row(&self, index: usize) -> Option<Vec<bool>> {
        self.step_results.get(index).cloned()
    }
    
    // Método para acceder a los resultados de un paso específico
    fn get_step_results(&self, step_index: usize) -> Option<Vec<bool>> {
        if step_index >= self.step_labels.len() {
            return None;
        }
        Some(self.step_results.iter().map(|row| row[step_index]).collect())
    }
}

/// Expresión booleana para Python
#[pyclass(name = "BooleanExpr")]
pub struct PyBooleanExpr {
    inner: BooleanExpr,
}

#[pymethods]
impl PyBooleanExpr {
    #[new]
    pub fn new(expression: &str) -> PyResult<Self> {
        let inner = BooleanExpr::new(expression)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyBooleanExpr { inner })
    }

    pub fn evaluate(&self, values: &Bound<'_, PyDict>) -> PyResult<bool> {
        let mut rust_values = HashMap::new();
        
        for (key, value) in values.iter() {
            let key_str: String = key.extract()?;
            let value_bool: bool = value.extract()?;
            rust_values.insert(key_str, value_bool);
        }
        
        // Convert HashMap<String, bool> to HashMap<&str, bool>
        let ref_map: HashMap<&str, bool> = rust_values.iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect();
        
        self.inner.evaluate(&ref_map)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    // CORREGIDO: Usar &Bound en lugar de Bound
    pub fn evaluate_with_defaults(&self, values: &Bound<'_, PyDict>, default: bool) -> bool {
        let mut rust_values = HashMap::new();
        
        for (key, value) in values.iter() {
            if let (Ok(key_str), Ok(value_bool)) = (key.extract::<String>(), value.extract::<bool>()) {
                rust_values.insert(key_str, value_bool);
            }
        }
        
        // Convert HashMap<String, bool> to HashMap<&str, bool>
        let ref_map: HashMap<&str, bool> = rust_values.iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect();

        self.inner.evaluate_with_defaults(&ref_map, default)
    }
    
    pub fn truth_table(&self) -> TruthTable {
        let table = self.inner.truth_table();
        let variables = self.inner.variables.clone();
        
        let mut combinations = Vec::new();
        let mut results = Vec::new();
        
        for (values, result) in table {
            let mut row = Vec::new();
            for var in &variables {
                row.push(*values.get(var.as_str()).unwrap());
            }
            combinations.push(row);
            results.push(result);
        }
        
        TruthTable {
            variables,
            combinations,
            results,
        }
    }

    pub fn full_truth_table(&self) -> FullTruthTable {
        // Get the full truth table as a Vec of (base, steps, result)
        let table = self.inner.full_truth_table();
        let base_variables = self.inner.variables.clone();

        // If the table is empty, return empty labels/results
        if table.is_empty() {
            return FullTruthTable {
                base_variables,
                step_labels: Vec::new(),
                step_results: Vec::new(),
            };
        }

        // Extract step labels from the first row's step HashMap
        let step_labels: Vec<String> = table[0].1.keys().cloned().collect();

        // Build step_results: each row is base values + step values + result
        let mut step_results = Vec::new();
        for (base_map, step_map, result) in table {
            let mut row = Vec::new();
            // Push base variable values in order
            for var in &base_variables {
                row.push(*base_map.get(var).unwrap_or(&false));
            }
            // Push step values in order of step_labels
            for label in &step_labels {
                row.push(*step_map.get(label).unwrap_or(&false));
            }
            // Push the final result
            row.push(result);
            step_results.push(row);
        }

        // Combine base_variables and step_labels for the full header
        let mut all_labels = base_variables.clone();
        all_labels.extend(step_labels.clone());
        all_labels.push("result".to_string());

        FullTruthTable {
            base_variables,
            step_labels: all_labels,
            step_results,
        }
    }
    
    // AÑADIDO: Método faltante
    pub fn to_prefix_notation(&self) -> String {
        self.inner.to_prefix_notation()
    }
    
    pub fn is_tautology(&self) -> bool {
        self.inner.is_tautology()
    }
    
    pub fn is_contradiction(&self) -> bool {
        self.inner.is_contradiction()
    }
    
    pub fn equivalent_to(&self, other: &Self) -> bool {  // CORREGIDO: usar &Self
        self.inner.equivalent_to(&other.inner)
    }
    
    #[getter]  // AÑADIDO: Usar getter para propiedades
    pub fn variables(&self) -> Vec<String> {
        self.inner.variables.clone()
    }
    
    pub fn complexity(&self) -> usize {
        self.inner.complexity()
    }
    
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    
    fn __repr__(&self) -> String {
        format!("BooleanExpr('{}')", self.inner.to_string())
    }
    
    fn __and__(&self, other: &Self) -> PyResult<Self> {  // CORREGIDO: usar &Self
        let new_expr = BooleanExpr::new(&format!("({}) & ({})", self.inner.to_string(), other.inner.to_string()))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyBooleanExpr { inner: new_expr })
    }
    
    fn __or__(&self, other: &Self) -> PyResult<Self> {  // CORREGIDO: usar &Self
        let new_expr = BooleanExpr::new(&format!("({}) | ({})", self.inner.to_string(), other.inner.to_string()))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyBooleanExpr { inner: new_expr })
    }
    
    fn __invert__(&self) -> PyResult<Self> {
        let new_expr = BooleanExpr::new(&format!("~({})", self.inner.to_string()))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyBooleanExpr { inner: new_expr })
    }
}

/// Función de utilidad para debugging: muestra el AST de una expresión
#[pyfunction]
fn parse_expression_debug(expression: &str) -> PyResult<String> {
    let expr = BooleanExpr::new(expression)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(expr.to_prefix_notation())
}

/// Función de utilidad: crea expresión desde tabla de verdad
#[pyfunction]
fn truth_table_from_expr(variables: Vec<String>, results: Vec<bool>) -> PyResult<PyBooleanExpr> {
    let expression = generate_expression_from_truth_table(&variables, &results);
    let inner = BooleanExpr::new(&expression)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(PyBooleanExpr { inner })
}

/// Función auxiliar para generar expresión desde tabla de verdad
fn generate_expression_from_truth_table(variables: &[String], results: &[bool]) -> String {
    let mut terms = Vec::new();
    let num_combinations = results.len();
    
    for i in 0..num_combinations {
        if results[i] {
            let mut term = String::new();
            for (j, var) in variables.iter().enumerate() {
                let value = (i >> (variables.len() - 1 - j)) & 1 == 1;
                if j > 0 {
                    term.push_str(" & ");
                }
                if !value {
                    term.push('~');
                }
                term.push_str(var);
            }
            terms.push(term);
        }
    }
    
    if terms.is_empty() {
        "false".to_string()
    } else if terms.len() == num_combinations {
        "true".to_string()
    } else {
        terms.join(" | ")
    }
}

/// Registra el módulo de álgebra booleana
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(parent.py(), "boolean_algebra")?;

    submodule.add_class::<PyBooleanExpr>()?;
    submodule.add_function(wrap_pyfunction!(parse_expression_debug, &submodule)?)?;
    submodule.add_function(wrap_pyfunction!(truth_table_from_expr, &submodule)?)?;

    parent.add_submodule(&submodule)?;
    
    Ok(())
}