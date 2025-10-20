use std::collections::HashSet;
use rust_xlsxwriter::{Format, FormatBorder, Workbook, Worksheet, XlsxError};
use serde_json::Value;
use crate::core::formatting::error::ExportError;

pub fn get_unique_headers(arr: &[Value]) -> Vec<&str> {
    let mut headers = HashSet::new();
    for item in arr {
        if let Value::Object(obj) = item {
            for key in obj.keys() {
                headers.insert(key.as_str());
            }
        }
    }
    let mut headers: Vec<&str> = headers.into_iter().collect();
    headers.sort();
    headers
}

pub fn separate_fields(obj: &serde_json::Map<String, Value>) -> (Vec<(&String, &Value)>, Vec<(&String, &Value)>) {
    let mut flat_fields = Vec::new();
    let mut nested_fields = Vec::new();

    for (key, value) in obj {
        match value {
            Value::Array(arr) if !arr.is_empty() => {
                if let Some(Value::Object(_)) = arr.first() {
                    nested_fields.push((key, value));
                } else {
                    flat_fields.push((key, value));
                }
            }
            Value::Object(_) => nested_fields.push((key, value)),
            Value::Array(_) => flat_fields.push((key, value)), // Array vacío
            _ => flat_fields.push((key, value)),
        }
    }

    (flat_fields, nested_fields)
}

pub fn value_to_string_readable(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::from("null"),
        Value::Array(arr) => {
            format!("[{}]", arr.iter()
                .map(value_to_string_readable)
                .collect::<Vec<_>>()
                .join(", "))
        }
        Value::Object(_) => String::from("{...}"),
    }
}

// ==================== IMPLEMENTACIÓN JERÁRQUICA MARKDOWN ====================

pub fn build_hierarchical_markdown(
    value: &Value,
    title: &str,
    level: usize,
) -> Result<String, ExportError> {
    let mut output = String::new();
    let hashes = "#".repeat(level);

    // Título de la sección
    output.push_str(&format!("{} {}\n\n", hashes, title));

    match value {
        Value::Object(obj) => {
            // Separar campos planos de campos anidados
            let (flat_fields, nested_fields) = separate_fields(obj);

            // Mostrar campos planos como lista
            if !flat_fields.is_empty() {
                for (key, value) in flat_fields {
                    output.push_str(&format!("- **{}**: {}\n", key, value_to_string_readable(value)));
                }
                output.push('\n');
            }

            // Procesar campos anidados
            for (key, value) in nested_fields {
                match value {
                    Value::Array(arr) if !arr.is_empty() => {
                        // Array de objetos -> crear sub-tabla
                        if let Some(Value::Object(_)) = arr.first() {
                            output.push_str(&format!("{}## {}\n\n", hashes, key));
                            let table = build_markdown_table(&arr)?;
                            output.push_str(&table);
                            output.push('\n');
                        } else {
                            // Array de valores simples -> mostrar como lista
                            output.push_str(&format!("- **{}**: [{}]\n",
                                                     key,
                                                     arr.iter()
                                                         .map(|v| value_to_string_readable(v))
                                                         .collect::<Vec<_>>()
                                                         .join(", ")));
                        }
                    }
                    Value::Object(nested_obj) => {
                        // Objeto anidado -> llamada recursiva
                        let nested_md = build_hierarchical_markdown(
                            &Value::Object(nested_obj.clone()),
                            &key,
                            level + 1
                        )?;
                        output.push_str(&nested_md);
                    }
                    _ => {
                        // Otros tipos anidados
                        output.push_str(&format!("- **{}**: {}\n", key, value_to_string_readable(value)));
                    }
                }
            }
        }
        Value::Array(arr) => {
            // Array en nivel raíz
            if !arr.is_empty() {
                if let Some(Value::Object(_)) = arr.first() {
                    let table = build_markdown_table(&arr)?;
                    output.push_str(&table);
                } else {
                    output.push_str("**Items**:\n\n");
                    for (i, item) in arr.iter().enumerate() {
                        output.push_str(&format!("{}. {}\n", i + 1, value_to_string_readable(item)));
                    }
                }
            } else {
                output.push_str("*No data available*\n");
            }
        }
        _ => {
            output.push_str(&format!("{}\n", value_to_string_readable(value)));
        }
    }

    Ok(output)
}


// ==================== IMPLEMENTACIÓN JERÁRQUICA EXCEL ====================

pub fn build_hierarchical_excel(
    value: &Value,
    path: &str,
    sheet_name: &str,
) -> Result<(), ExportError>  {
    let mut workbook = Workbook::new();

    // Crear formatos
    let header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("D3D3D3");

    let cell_format = Format::new()
        .set_border(FormatBorder::Thin);

    let title_format = Format::new()
        .set_bold()
        .set_font_size(14);

    // Procesar recursivamente
    let mut sheet_count = 0;
    process_value_for_excel(
        value,
        &mut workbook,
        &mut sheet_count,
        sheet_name,
        &header_format,
        &cell_format,
        &title_format,
    )?;

    workbook.save(path)?;
    Ok(())
}

pub fn process_value_for_excel(
    value: &Value,
    workbook: &mut Workbook,
    sheet_count: &mut usize,
    sheet_name: &str,
    header_format: &Format,
    cell_format: &Format,
    title_format: &Format,
) -> Result<(), XlsxError> {
    match value {
        Value::Object(obj) => {
            // Crear nueva hoja
            let worksheet_name = if *sheet_count == 0 {
                sheet_name.to_string()
            } else {
                format!("{} {}", sheet_name, sheet_count)
            };

            let mut worksheet = workbook.add_worksheet().set_name(&worksheet_name)?;
            *sheet_count += 1;

            let mut row = 0;

            // Separar campos planos y anidados
            let (flat_fields, nested_fields) = separate_fields(obj);

            // Escribir campos planos como tabla clave-valor
            if !flat_fields.is_empty() {
                worksheet.write_string_with_format(row, 0, "Property", header_format)?;
                worksheet.write_string_with_format(row, 1, "Value", header_format)?;
                row += 1;

                for (key, value) in flat_fields {
                    worksheet.write_string(row, 0, key)?;
                    write_value_to_excel_cell(&mut worksheet, row, 1, value, cell_format)?;
                    row += 1;
                }
                row += 1; // Espacio extra
            }

            // Procesar campos anidados, pero recolectar objetos anidados para procesarlos después
            let mut nested_object_fields: Vec<(String, serde_json::Map<String, Value>)> = Vec::new();

            for (key, nested_value) in nested_fields {
                match nested_value {
                    Value::Array(arr) if !arr.is_empty() => {
                        // Escribir título de la sección
                        worksheet.write_string_with_format(row, 0, &format!("{}:", key), title_format)?;
                        row += 1;

                        if let Some(Value::Object(_)) = arr.first() {
                            // Array de objetos -> escribir como tabla
                            let headers = get_unique_headers(&arr);

                            // Escribir headers
                            for (col, header) in headers.iter().enumerate() {
                                worksheet.write_string_with_format(row, col as u16, *header, header_format)?;
                            }
                            row += 1;

                            // Escribir datos
                            for (item_row, item) in arr.iter().enumerate() {
                                if let Value::Object(item_obj) = item {
                                    for (col, header) in headers.iter().enumerate() {
                                        if let Some(value) = item_obj.get(*header) {
                                            write_value_to_excel_cell(
                                                &mut worksheet,
                                                row + item_row as u32,
                                                col as u16,
                                                value,
                                                cell_format,
                                            )?;
                                        }
                                    }
                                }
                            }
                            row += arr.len() as u32 + 1;
                        } else {
                            // Array de valores simples
                            for (i, item) in arr.iter().enumerate() {
                                worksheet.write_string(row + i as u32, 0, &format!("[{}]", i + 1))?;
                                write_value_to_excel_cell(&mut worksheet, row + i as u32, 1, item, cell_format)?;
                            }
                            row += arr.len() as u32 + 1;
                        }
                    }
                    Value::Object(nested_obj) => {
                        // Recolectar objeto anidado para procesarlo después (evita borrows mutables solapados)
                        nested_object_fields.push((key.clone(), nested_obj.clone()));
                    }
                    _ => {
                        // Otros tipos anidados
                        worksheet.write_string(row, 0, key)?;
                        write_value_to_excel_cell(&mut worksheet, row, 1, nested_value, cell_format)?;
                        row += 1;
                    }
                }
            }

            // Autoajustar columnas
            worksheet.autofit();

            // Soltar la referencia del worksheet antes de crear nuevas hojas en el workbook
            drop(worksheet);

            // Procesar objetos anidados fuera del borrow mutable del worksheet
            for (key, nested_map) in nested_object_fields {
                let nested_sheet_name = format!("{} - {}", sheet_name, key);
                process_value_for_excel(
                    &Value::Object(nested_map),
                    workbook,
                    sheet_count,
                    &nested_sheet_name,
                    header_format,
                    cell_format,
                    title_format,
                )?;
            }
        }
        Value::Array(arr) => {
            // Array en nivel raíz
            let mut worksheet = workbook.add_worksheet().set_name(sheet_name)?;

            if !arr.is_empty() {
                if let Some(Value::Object(_)) = arr.first() {
                    write_array_to_excel_worksheet(&arr, &mut worksheet, header_format, cell_format)?;
                } else {
                    // Array de valores simples
                    worksheet.write_string_with_format(0, 0, "Items", header_format)?;
                    for (i, item) in arr.iter().enumerate() {
                        worksheet.write_number(i as u32 + 1, 0, (i + 1) as f64)?;
                        write_value_to_excel_cell(&mut worksheet, i as u32 + 1, 1, item, cell_format)?;
                    }
                }
            } else {
                worksheet.write_string(0, 0, "No data available")?;
            }

            worksheet.autofit();
        }
        _ => {
            // Valor simple
            let mut worksheet = workbook.add_worksheet().set_name(sheet_name)?;
            write_value_to_excel_cell(&mut worksheet, 0, 0, value, cell_format)?;
            worksheet.autofit();
        }
    }

    Ok(())
}

// ==================== MÉTODOS HELPER MEJORADOS ====================

pub fn build_markdown_table(data: &[Value]) -> Result<String, ExportError> {
    if data.is_empty() {
        return Ok(String::from("# Empty Data\n\nNo hay datos para mostrar"));
    }

    let mut output = String::new();
    let mut headers: Vec<&str> = vec![];

    // Obtener todos los headers únicos
    for item in data {
        if let Value::Object(obj) = item {
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
        if let Value::Object(obj) = item {
            let row: Vec<String> = headers.iter()
                .map(|&h| obj.get(h)
                    .map(|v| value_to_string_readable(v))
                    .unwrap_or_else(|| String::from("")))
                .collect();
            output.push_str(&format!("| {} |\n", row.join(" | ")));
        }
    }

    Ok(output)
}


pub fn write_array_to_excel_worksheet(
    arr: &[Value],
    worksheet: &mut Worksheet,
    header_format: &Format,
    cell_format: &Format,
) -> Result<(), XlsxError> {
    if arr.is_empty() {
        worksheet.write_string(0, 0, "No data available")?;
        return Ok(());
    }

    let headers = get_unique_headers(arr);

    // Escribir headers
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, header_format)?;
    }

    // Escribir datos
    for (row, item) in arr.iter().enumerate() {
        if let Value::Object(obj) = item {
            for (col, header) in headers.iter().enumerate() {
                if let Some(value) = obj.get(*header) {
                    write_value_to_excel_cell(worksheet, (row + 1) as u32, col as u16, value, cell_format)?;
                }
            }
        }
    }

    // Autoajustar columnas
    for col in 0..headers.len() {
        worksheet.set_column_width(col as u16, 15.0)?;
    }

    Ok(())
}

pub fn write_value_to_excel_cell(
    worksheet: &mut Worksheet,
    row: u32,
    col: u16,
    value: &Value,
    format: &Format,
) -> Result<(), XlsxError> {
    match value {
        Value::String(s) => {
            worksheet.write_string_with_format(row, col, s, format)?;
            Ok(())
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                worksheet.write_number_with_format(row, col, i as f64, format)?;
            } else if let Some(f) = n.as_f64() {
                worksheet.write_number_with_format(row, col, f, format)?;
            } else {
                worksheet.write_string_with_format(row, col, &n.to_string(), format)?;
            }
            Ok(())
        }
        Value::Bool(b) => {
            worksheet.write_boolean_with_format(row, col, *b, format)?;
            Ok(())
        }
        Value::Null => {
            worksheet.write_string_with_format(row, col, "", format)?;
            Ok(())
        }
        Value::Array(arr) => {
            let arr_str = arr.iter()
                .map(|v| value_to_string_readable(v))
                .collect::<Vec<_>>()
                .join(", ");
            worksheet.write_string_with_format(row, col, &format!("[{}]", arr_str), format)?;
            Ok(())
        }
        Value::Object(_) => {
            worksheet.write_string_with_format(row, col, "{...}", format)?;
            Ok(())
        }
    }
}


// Mantener el método original para compatibilidad
pub fn value_to_string(value: &Value) -> String {
    value_to_string_readable(value)
}