use std::collections::HashSet;
use csv::Writer;
use serde::Serialize;
use serde_json::{to_string_pretty, to_value, Value};
use std::error::Error;
use std::fmt;
use rust_xlsxwriter::{Format, FormatBorder, Workbook, XlsxError};
use crate::core::formatting::error::ExportError;


/// Trait que proporciona métodos para exportar datos serializable a diferentes formatos.
pub trait Exportable: Serialize {
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

    /// Exporta el objeto a una tabla Markdown.
    fn to_markdown(&self) -> Result<String, ExportError> {
        let value = to_value(self)?;
        match value {
            serde_json::Value::Array(arr) if !arr.is_empty() => Self::build_markdown_table(&arr),
            serde_json::Value::Array(arr) if arr.is_empty() => Ok(String::from("Empty Data")),
            serde_json::Value::Object(obj) => Self::build_markdown_table(&[serde_json::Value::Object(obj)]),
            _ => Ok(String::from("# Data\n\nNo se puede convertir a tabla markdown")),
        }
    }

    /// Exporta el objeto a un archivo Excel.
    fn to_excel(&self, path: &str) -> Result<(), ExportError> {
        let value = serde_json::to_value(self)?;
        let mut workbook = Workbook::new();

        // Crear formato para el header
        let header_format = Format::new()
            .set_bold()
            .set_border(FormatBorder::Thin)
            .set_background_color("D3D3D3");

        // Crear formato para las celdas normales
        let cell_format = Format::new()
            .set_border(FormatBorder::Thin);

        let worksheet = workbook.add_worksheet();

        match value {
            Value::Array(arr) => {
                if arr.is_empty() {
                    worksheet.write_string(0, 0, "No data available")?;
                } else {
                    Self::write_array_to_excel(&arr, worksheet, &header_format, &cell_format)?;
                }
            }
            Value::Object(obj) => {
                Self::write_object_to_excel(&obj, worksheet, &header_format, &cell_format)?;
            }
            _ => {
                worksheet.write_string(0, 0, &format!("Value: {}", Self::value_to_string(&value)))?;
            }
        }

        workbook.save(path)?;
        Ok(())
    }

    fn write_array_to_excel(
        arr: &[Value],
        worksheet: &mut rust_xlsxwriter::Worksheet,
        header_format: &Format,
        cell_format: &Format,
    ) -> Result<(), XlsxError> {
        if arr.is_empty() {
            return Ok(());
        }

        // Obtener todos los headers únicos
        let mut headers = HashSet::new();
        for item in arr {
            if let Value::Object(obj) = item {
                for key in obj.keys() {
                    headers.insert(key.clone());
                }
            }
        }

        let headers: Vec<String> = headers.into_iter().collect();

        // Escribir headers
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, col as u16, header, header_format)?;
        }

        // Escribir datos
        for (row, item) in arr.iter().enumerate() {
            if let Value::Object(obj) = item {
                for (col, header) in headers.iter().enumerate() {
                    if let Some(value) = obj.get(header) {
                        Self::write_value_to_excel(worksheet, (row + 1) as u32, col as u16, value, cell_format)?;
                    }
                }
            }
        }

        // Autoajustar el ancho de las columnas
        for col in 0..headers.len() {
            worksheet.set_column_width(col as u16, 15.0)?;
        }

        Ok(())
    }

    fn write_object_to_excel(
        obj: &serde_json::Map<String, Value>,
        worksheet: &mut rust_xlsxwriter::Worksheet,
        header_format: &Format,
        cell_format: &Format,
    ) -> Result<(), XlsxError> {
        let headers: Vec<&String> = obj.keys().collect();

        // Escribir headers
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, col as u16, *header, header_format)?;
        }

        // Escribir datos
        for (col, header) in headers.iter().enumerate() {
            if let Some(value) = obj.get(*header) {
                Self::write_value_to_excel(worksheet, 1, col as u16, value, cell_format)?;
            }
        }

        // Autoajustar el ancho de las columnas
        for col in 0..headers.len() {
            worksheet.set_column_width(col as u16, 15.0)?;
        }

        Ok(())
    }

    fn write_value_to_excel(
        worksheet: &mut rust_xlsxwriter::Worksheet,
        row: u32,
        col: u16,
        value: &Value,
        format: &Format,
    ) -> Result<(), XlsxError> {
        match value {
            Value::String(s) => {
                worksheet.write_string_with_format(row, col, s, format)?;
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    worksheet.write_number_with_format(row, col, i as f64, format)?;
                } else if let Some(f) = n.as_f64() {
                    worksheet.write_number_with_format(row, col, f, format)?;
                } else {
                    worksheet.write_string_with_format(row, col, &n.to_string(), format)?;
                }
            }
            Value::Bool(b) => {
                worksheet.write_boolean_with_format(row, col, *b, format)?;
            }
            Value::Null => {
                worksheet.write_string_with_format(row, col, "", format)?;
            }
            _ => {
                worksheet.write_string_with_format(row, col, &Self::value_to_string(value), format)?;
            }
        }

        Ok(())
    }

    fn build_markdown_table(data: &[serde_json::Value]) -> Result<String, ExportError> {
        if data.is_empty() {
            return Ok(String::from("# Empty Data\n\nNo hay datos para mostrar"));
        }

        let mut output = String::from("# Data Export\n\n");
        let mut headers: Vec<&str> = vec![];

        // Obtener todos los headers únicos
        for item in data {
            if let serde_json::Value::Object(obj) = item {
                for key in obj.keys() {
                    if !headers.contains(&key.as_str()) {
                        headers.push(key);
                    }
                }
            }
        }

        if headers.is_empty() {
            return Ok(String::from("# No valid data\n\nNo se encontraron objetos válidos"));
        }

        // Header de la tabla
        output.push_str(&format!("| {} |\n", headers.join(" | ")));

        // Separador
        output.push_str(&format!("|{}|\n", vec!["---"; headers.len()].join("|")));

        // Filas de datos
        for item in data {
            if let serde_json::Value::Object(obj) = item {
                let row: Vec<String> = headers.iter()
                    .map(|&h| obj.get(h).map(|v| Self::value_to_string(v)).unwrap_or_else(|| String::from("")))
                    .collect();
                output.push_str(&format!("| {} |\n", row.join(" | ")));
            }
        }

        Ok(output)
    }

    fn value_to_string(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => String::from("null"),
            serde_json::Value::Array(_) => String::from("[array]"),
            serde_json::Value::Object(_) => String::from("[object]"),
        }
    }
}

impl<T> Exportable for T where T: Serialize {}

