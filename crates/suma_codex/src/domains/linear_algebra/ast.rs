use serde::Serialize;

// 1. El Modelo Raíz (Lo que el Parser devuelve)
#[derive(Debug, Serialize, Clone)]
pub struct LinearAlgebraModel {
    pub name: Option<String>, // Nombre del bloque contenedor (ej: "Tarea_1")
    pub actions: Vec<LinearAlgebraAction>, // Lista de Sistemas o Análisis
}

// 2. Las Acciones Posibles
#[derive(Debug, Serialize, Clone)]
pub enum LinearAlgebraAction {
    System(SystemDef),
    Analysis(AnalysisDef),
}

// 3. Definición de Sistema (Artefacto)
#[derive(Debug, Serialize, Clone)]
pub struct SystemDef {
    pub name: String,
    pub matrix_a: Option<MatrixData>,
    pub vector_b: Option<MatrixData>,
}

// 4. Definición de Análisis (Query)
#[derive(Debug, Serialize, Clone)]
pub struct AnalysisDef {
    pub name: String,
    pub target: String,
    pub calculate: Vec<String>,
    pub export: Vec<(String, String)>, // (Propiedad -> Variable)
}

// 5. Estructura de Datos Matriciales
#[derive(Debug, Serialize, Clone)]
pub struct MatrixData {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}