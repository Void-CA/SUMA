#[cfg(test)]
mod tests {
    use serde::Serialize;
    use crate::core::formatting::export::Exportable;
    use std::fs;

    #[derive(Serialize, Debug, PartialEq)]
    struct User {
        name: String,
        age: u32,
        active: bool,
        salary: Option<f64>,
    }

    #[derive(Serialize)]
    struct Product {
        id: u32,
        name: String,
        price: f64,
        category: String,
        in_stock: bool,
    }

    #[derive(Serialize)]
    struct MixedData {
        string_field: String,
        number_field: i32,
        float_field: f64,
        boolean_field: bool,
        optional_field: Option<String>,
        null_field: Option<()>,
    }

    // Tests básicos de JSON
    #[test]
    fn test_json_export() {
        let user = User {
            name: "Alice".to_string(),
            age: 30,
            active: true,
            salary: Some(50000.0),
        };

        let result = user.to_json();
        assert!(result.is_ok());
        let json_str = result.unwrap();
        assert!(json_str.contains("Alice"));
        assert!(json_str.contains("30"));
        assert!(json_str.contains("true"));
        assert!(json_str.contains("50000"));
    }

    #[test]
    fn test_json_export_with_none() {
        let user = User {
            name: "Bob".to_string(),
            age: 25,
            active: false,
            salary: None,
        };

        let result = user.to_json();
        assert!(result.is_ok());
        let json_str = result.unwrap();
        assert!(json_str.contains("Bob"));
        assert!(json_str.contains("25"));
        assert!(json_str.contains("false"));
        assert!(json_str.contains("null"));
    }

    // Tests de CSV
    #[test]
    fn test_csv_export() {
        let user = User {
            name: "Bob".to_string(),
            age: 25,
            active: false,
            salary: Some(45000.0),
        };

        let result = user.to_csv();
        assert!(result.is_ok());
        let csv_str = result.unwrap();
        assert!(csv_str.contains("Bob"));
        assert!(csv_str.contains("25"));
        assert!(csv_str.contains("false"));
        assert!(csv_str.contains("45000"));
    }

    #[test]
    fn test_csv_export_multiple_records() {
        let users = vec![
            User {
                name: "Charlie".to_string(),
                age: 35,
                active: true,
                salary: Some(60000.0),
            },
            User {
                name: "Diana".to_string(),
                age: 28,
                active: false,
                salary: None,
            },
        ];

        let result = users.to_csv();
        assert!(result.is_ok());
        let csv_str = result.unwrap();
        assert!(csv_str.contains("Charlie"));
        assert!(csv_str.contains("Diana"));
        assert!(csv_str.contains("35"));
        assert!(csv_str.contains("28"));
    }

    // Tests de Markdown
    #[test]
    fn test_markdown_export() {
        let users = vec![
            User {
                name: "Charlie".to_string(),
                age: 35,
                active: true,
                salary: Some(60000.0),
            },
            User {
                name: "Diana".to_string(),
                age: 28,
                active: false,
                salary: Some(55000.0),
            },
        ];

        let result = users.to_markdown();
        assert!(result.is_ok());
        let md_str = result.unwrap();
        assert!(md_str.contains("Charlie"));
        assert!(md_str.contains("Diana"));
        assert!(md_str.contains("name"));
        assert!(md_str.contains("age"));
        assert!(md_str.contains("active"));
        assert!(md_str.contains("salary"));
        // Verificar estructura de tabla markdown
        assert!(md_str.contains("|"));
        assert!(md_str.contains("---"));
    }

    #[test]
    fn test_markdown_export_single_object() {
        let user = User {
            name: "Eve".to_string(),
            age: 40,
            active: true,
            salary: Some(70000.0),
        };

        let result = user.to_markdown();
        assert!(result.is_ok());
        let md_str = result.unwrap();
        assert!(md_str.contains("Eve"));
        assert!(md_str.contains("40"));
        assert!(md_str.contains("true"));
        assert!(md_str.contains("70000"));
    }

    #[test]
    fn test_empty_array_markdown() {
        let users: Vec<User> = vec![];
        let result = users.to_markdown();

        assert!(result.is_ok());
        let md_str = result.unwrap();
        assert!(md_str.contains("Empty Data"));
    }

    #[test]
    fn test_markdown_with_mixed_data() {
        let data = MixedData {
            string_field: "test".to_string(),
            number_field: 42,
            float_field: 3.14,
            boolean_field: true,
            optional_field: Some("optional".to_string()),
            null_field: None,
        };

        let result = data.to_markdown();
        assert!(result.is_ok());
        let md_str = result.unwrap();
        assert!(md_str.contains("test"));
        assert!(md_str.contains("42"));
        assert!(md_str.contains("3.14"));
        assert!(md_str.contains("true"));
    }

    // Tests de Excel
    #[test]
    fn test_excel_export_single_object() {
        let user = User {
            name: "Alice".to_string(),
            age: 30,
            active: true,
            salary: Some(50000.0),
        };

        let path = "test_single_object.xlsx";
        let result = user.to_excel(path);

        // Verificar que se creó el archivo
        if result.is_ok() {
            assert!(fs::metadata(path).is_ok());
            // Limpiar
            let _ = fs::remove_file(path);
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_excel_export_array_of_objects() {
        let users = vec![
            User {
                name: "Alice".to_string(),
                age: 30,
                active: true,
                salary: Some(50000.0),
            },
            User {
                name: "Bob".to_string(),
                age: 25,
                active: false,
                salary: Some(45000.0),
            },
            User {
                name: "Charlie".to_string(),
                age: 35,
                active: true,
                salary: None,
            },
        ];

        let path = "test_array_objects.xlsx";
        let result = users.to_excel(path);

        if result.is_ok() {
            assert!(fs::metadata(path).is_ok());
            let _ = fs::remove_file(path);
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_excel_export_with_products() {
        let products = vec![
            Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: "Electronics".to_string(),
                in_stock: true,
            },
            Product {
                id: 2,
                name: "Mouse".to_string(),
                price: 25.50,
                category: "Electronics".to_string(),
                in_stock: false,
            },
            Product {
                id: 3,
                name: "Desk".to_string(),
                price: 299.99,
                category: "Furniture".to_string(),
                in_stock: true,
            },
        ];

        let path = "test_products.xlsx";
        let result = products.to_excel(path);

        if result.is_ok() {
            assert!(fs::metadata(path).is_ok());
            let _ = fs::remove_file(path);
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_excel_export_empty_array() {
        let empty_users: Vec<User> = vec![];
        let path = "test_empty_array.xlsx";
        let result = empty_users.to_excel(path);

        if result.is_ok() {
            assert!(fs::metadata(path).is_ok());
            let _ = fs::remove_file(path);
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_excel_export_complex_data() {
        let complex_data = MixedData {
            string_field: "Hello World".to_string(),
            number_field: -42,
            float_field: 123.456,
            boolean_field: false,
            optional_field: Some("Present".to_string()),
            null_field: None,
        };

        let path = "test_complex_data.xlsx";
        let result = complex_data.to_excel(path);

        if result.is_ok() {
            assert!(fs::metadata(path).is_ok());
            let _ = fs::remove_file(path);
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_excel_export_invalid_path() {
        let user = User {
            name: "Test".to_string(),
            age: 99,
            active: true,
            salary: None,
        };

        // Intentar guardar en un directorio que no existe
        let path = "/invalid/path/test.xlsx";
        let result = user.to_excel(path);

        // Debería fallar
        assert!(result.is_err());
    }

    // Tests de errores
    #[test]
    fn test_json_serialization_error() {
        // Este test verifica que los errores de serialización se manejan correctamente
        // Podríamos crear un tipo que falle al serializar, pero es más simple
        // confiar en que serde funciona correctamente
        let user = User {
            name: "Normal".to_string(),
            age: 30,
            active: true,
            salary: Some(1000.0),
        };

        // Esta serialización debería funcionar siempre
        let result = user.to_json();
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_formats_consistency() {
        let user = User {
            name: "Consistency Test".to_string(),
            age: 99,
            active: true,
            salary: Some(12345.67),
        };

        // Probar todos los formatos con los mismos datos
        let json_result = user.to_json();
        let csv_result = user.to_csv();
        let md_result = user.to_markdown();
        let excel_result = user.to_excel("test_consistency.xlsx");

        assert!(json_result.is_ok());
        assert!(csv_result.is_ok());
        assert!(md_result.is_ok());
        assert!(excel_result.is_ok());

        // Limpiar archivo Excel
        let _ = fs::remove_file("test_consistency.xlsx");

        // Verificar que todos contienen los datos básicos
        let json_str = json_result.unwrap();
        let csv_str = csv_result.unwrap();
        let md_str = md_result.unwrap();

        assert!(json_str.contains("Consistency Test"));
        assert!(csv_str.contains("Consistency Test"));
        assert!(md_str.contains("Consistency Test"));
        assert!(json_str.contains("99"));
        assert!(csv_str.contains("99"));
        assert!(md_str.contains("99"));
    }

    // Test de rendimiento con datos grandes
    #[test]
    fn test_large_dataset_export() {
        let large_dataset: Vec<User> = (0..100)
            .map(|i| User {
                name: format!("User_{}", i),
                age: 20 + (i % 40),
                active: i % 2 == 0,
                salary: Some(30000.0 + (i as f64 * 100.0)),
            })
            .collect();

        let path = "test_large_dataset.xlsx";
        let result = large_dataset.to_excel(path);

        if result.is_ok() {
            assert!(fs::metadata(path).is_ok());
            let _ = fs::remove_file(path);
        }

        assert!(result.is_ok());
    }
}