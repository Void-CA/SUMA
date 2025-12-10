use csv::Writer;
use serde::Serialize;
use serde_json::{to_string_pretty, to_value, Value};
use crate::formatting::error::ExportError;
use crate::formatting::utils::{build_hierarchical_excel, build_hierarchical_markdown, build_markdown_table};

/// Trait que proporciona métodos para exportar datos serializable a diferentes formatos.
pub trait Exportable: Serialize {
    // ==================== MÉTODOS EXISTENTES ====================
    
    /// Exporta el objeto a una cadena JSON con formato legible.
    fn to_json(&self) -> Result<String, ExportError> {
        Ok(to_string_pretty(self)?)
    }

    /// Exporta el objeto a una cadena CSV.
    fn to_csv(&self) -> Result<String, ExportError> {
        let mut wtr = Writer::from_writer(vec![]);
        wtr.serialize(self)?;
        let data = wtr.into_inner()?;
        Ok(String::from_utf8(data)?)
    }

    /// Exporta el objeto a una tabla Markdown (versión plana - compatibilidad).
    fn to_markdown(&self) -> Result<String, ExportError> {
        let value = to_value(self)?;
        match value {
            Value::Array(arr) if !arr.is_empty() => build_markdown_table(&arr),
            Value::Array(arr) if arr.is_empty() => Ok(String::from("# Empty Data")),
            Value::Object(obj) => build_markdown_table(&[Value::Object(obj)]),
            _ => Ok(String::from("# Data\n\nNo se puede convertir a tabla markdown")),
        }
    }

    /// Exporta el objeto a un archivo Excel (versión plana - compatibilidad).
    fn to_excel(&self, path: &str) -> Result<(), ExportError> where Self: Sized {
        let value = serde_json::to_value(self)?;
        build_hierarchical_excel(&value, path, "Data")
    }

    // ==================== MÉTODOS JERÁRQUICOS NUEVOS ====================

    /// Exporta el objeto a Markdown con estructura jerárquica
    fn to_markdown_hierarchical(&self) -> Result<String, ExportError> {
        let value = to_value(self)?;
        build_hierarchical_markdown(&value, "Data Export", 1)
    }
    
}

// Utils functions

impl<T> Exportable for T where T: Serialize {}