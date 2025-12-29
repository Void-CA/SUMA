use serde::{Serialize, Deserialize};

// ==========================================
// 1. EL MODELO RAÍZ (Contenedor del archivo)
// ==========================================
// Ahora es una lista plana de instrucciones (Definiciones o Queries)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinearAlgebraBlock {
    pub statements: Vec<LinAlgStmt>, 
}

// ==========================================
// 2. ENUM PRINCIPAL: ¿Estado o Ejecución?
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinAlgStmt {
    System(SystemDef), // Creación de modelo (LinearSystem)
    Query(QueryDef),   // Ejecución (query)
}

// ==========================================
// 3. DEFINICIÓN DE SISTEMA (Estado)
// ==========================================
// Mapea: LinearSystem "ID" { ... }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemDef {
    pub id: String, // El nombre entre comillas
    // Renombramos para coincidir con la gramática (coefficients/constants)
    // Usamos Option porque el usuario podría definir solo A sin b temporalmente
    pub coefficients: Option<MatrixData>, 
    pub constants: Option<MatrixData>,    
}

// ==========================================
// 4. DEFINICIÓN DE QUERY (Ejecución) - ¡NUEVO!
// ==========================================
// Mapea: query "ID" { capability as alias }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryDef {
    pub target_id: String,      // A quién preguntamos (ej: "Produccion")
    pub requests: Vec<Request>, // Lista de lo que queremos saber
}

// Un item individual dentro de la query
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub capability: Capability, // Qué operación (Enum seguro)
    pub alias: Option<String>,  // Nombre de salida (sugar: as 'x')
}

// Enum estricto para evitar strings mágicos "determinant"
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")] // Para que serialize como "determinant"
pub enum Capability {
    Determinant,
    Solution,
    Inverse,
    Rank,
    Trace,
}

// ==========================================
// 5. ESTRUCTURAS DE DATOS (Primitivas)
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixData {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}