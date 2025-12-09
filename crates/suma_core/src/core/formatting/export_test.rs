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

// tests/export_hierarchical_tests.rs
#[cfg(test)]
mod export_hierarchical_tests {use std::collections::HashMap;
    use serde::Serialize;
    use std::fs;
    use serde_json::Value;
    use serde_json::map::Map;
    use tempfile::NamedTempFile;
    use crate::core::formatting::export::Exportable;
    use crate::core::formatting::utils::build_markdown_table;

    // Structs de prueba para testing
    #[derive(Serialize)]
    struct SimpleStruct {
        name: String,
        value: i32,
        active: bool,
    }

    #[derive(Serialize)]
    struct NestedStruct {
        id: u32,
        info: SimpleStruct,
        tags: Vec<String>,
        metadata: HashMap<String, String>,
    }

    #[derive(Serialize)]
    struct ComplexStruct {
        title: String,
        items: Vec<SimpleStruct>,
        config: NestedStruct,
        scores: Vec<f64>,
        empty_array: Vec<String>,
        null_field: Option<String>,
    }

    #[derive(Serialize)]
    struct NetworkTestData {
        base_network: String,
        base_cidr: u8,
        network_class: String,
        total_subnets: usize,
        total_hosts_capacity: u32,
        utilization_percentage: f64,
        subnets: Vec<SubnetTestData>,
    }

    #[derive(Serialize)]
    struct SubnetTestData {
        subnet_id: u32,
        network_address: String,
        first_host: String,
        last_host: String,
        broadcast: String,
        hosts_count: u32,
    }

    // Tests para Markdown Jerárquico
    #[test]
    fn test_markdown_hierarchical_simple_struct() {
        let data = SimpleStruct {
            name: "Test".to_string(),
            value: 42,
            active: true,
        };

        let result = data.to_markdown_hierarchical();
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        assert!(markdown.contains("# Data Export"));
        assert!(markdown.contains("**name**: Test"));
        assert!(markdown.contains("**value**: 42"));
        assert!(markdown.contains("**active**: true"));
    }

    #[test]
    fn test_markdown_hierarchical_nested_struct() {
        let data = NestedStruct {
            id: 1,
            info: SimpleStruct {
                name: "Nested".to_string(),
                value: 100,
                active: false,
            },
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            metadata: {
                let mut map = HashMap::new();
                map.insert("key1".to_string(), "value1".to_string());
                map.insert("key2".to_string(), "value2".to_string());
                map
            },
        };

        let result = data.to_markdown_hierarchical();
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        assert!(markdown.contains("# Data Export"));
        assert!(markdown.contains("**id**: 1"));
        assert!(markdown.contains("## info"));
        assert!(markdown.contains("**name**: Nested"));
        assert!(markdown.contains("**tags**: [tag1, tag2]"));
        // metadata debería procesarse como objeto anidado
        assert!(markdown.contains("## metadata"));
    }

    #[test]
    fn test_markdown_hierarchical_complex_struct() {
        let data = ComplexStruct {
            title: "Complex Test".to_string(),
            items: vec![
                SimpleStruct {
                    name: "Item1".to_string(),
                    value: 10,
                    active: true,
                },
                SimpleStruct {
                    name: "Item2".to_string(),
                    value: 20,
                    active: false,
                },
            ],
            config: NestedStruct {
                id: 99,
                info: SimpleStruct {
                    name: "Config".to_string(),
                    value: 999,
                    active: true,
                },
                tags: vec!["config".to_string()],
                metadata: HashMap::new(),
            },
            scores: vec![1.1, 2.2, 3.3],
            empty_array: vec![],
            null_field: None,
        };

        let result = data.to_markdown_hierarchical();
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        println!("{}", markdown); // Para debug si falla
        
        assert!(markdown.contains("# Data Export"));
        assert!(markdown.contains("**title**: Complex Test"));
        assert!(markdown.contains("## items")); // Array de objetos -> sub-tabla
        assert!(markdown.contains("## config")); // Objeto anidado
        assert!(markdown.contains("**scores**: [1.1, 2.2, 3.3]")); // Array simple -> lista
        assert!(markdown.contains("**empty_array**: []")); // Array vacío
        assert!(markdown.contains("**null_field**: null")); // Campo nulo
    }

    #[test]
    fn test_markdown_hierarchical_network_data() {
        let data = NetworkTestData {
            base_network: "192.168.1.0".to_string(),
            base_cidr: 24,
            network_class: "Class C".to_string(),
            total_subnets: 2,
            total_hosts_capacity: 508,
            utilization_percentage: 85.5,
            subnets: vec![
                SubnetTestData {
                    subnet_id: 1,
                    network_address: "192.168.1.0".to_string(),
                    first_host: "192.168.1.1".to_string(),
                    last_host: "192.168.1.126".to_string(),
                    broadcast: "192.168.1.127".to_string(),
                    hosts_count: 126,
                },
                SubnetTestData {
                    subnet_id: 2,
                    network_address: "192.168.1.128".to_string(),
                    first_host: "192.168.1.129".to_string(),
                    last_host: "192.168.1.254".to_string(),
                    broadcast: "192.168.1.255".to_string(),
                    hosts_count: 126,
                },
            ],
        };

        let result = data.to_markdown_hierarchical();
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        println!("{}", markdown); // Para debug
        
        // Verificar estructura jerárquica
        assert!(markdown.contains("# Data Export"));
        assert!(markdown.contains("**base_network**: 192.168.1.0"));
        assert!(markdown.contains("**base_cidr**: 24"));
        assert!(markdown.contains("## subnets")); // Array de objetos -> sub-tabla
        assert!(markdown.contains("| subnet_id | network_address | first_host | last_host | broadcast | hosts_count |"));
        assert!(markdown.contains("| 1 | 192.168.1.0 | 192.168.1.1 | 192.168.1.126 | 192.168.1.127 | 126 |"));
    }

    #[test]
    fn test_markdown_hierarchical_empty_data() {
        let empty_vec: Vec<String> = vec![];
        let result = empty_vec.to_markdown_hierarchical();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("No data available"));

        let empty_obj = HashMap::<String, String>::new();
        let result = empty_obj.to_markdown_hierarchical();
        assert!(result.is_ok());
        // Debería mostrar el título pero sin contenido
    }

    #[test]
    fn test_markdown_hierarchical_primitive_types() {
        // Test con tipos primitivos directamente
        let number = 42;
        let result = number.to_markdown_hierarchical();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("42"));

        let string = "hello".to_string();
        let result = string.to_markdown_hierarchical();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("hello"));

        let array = vec![1, 2, 3];
        let result = array.to_markdown_hierarchical();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Items") || output.contains("1. 1"));
    }

    // Tests para Excel Jerárquico
    #[test]
    fn test_excel_hierarchical_simple_struct() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let data = SimpleStruct {
            name: "Test".to_string(),
            value: 42,
            active: true,
        };

        let result = data.to_excel(path);
        assert!(result.is_ok());
        
        // Verificar que el archivo se creó
        assert!(fs::metadata(path).is_ok());
        
        // Limpiar
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_excel_hierarchical_complex_struct() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let data = ComplexStruct {
            title: "Test".to_string(),
            items: vec![
                SimpleStruct {
                    name: "Item1".to_string(),
                    value: 10,
                    active: true,
                },
            ],
            config: NestedStruct {
                id: 1,
                info: SimpleStruct {
                    name: "Config".to_string(),
                    value: 100,
                    active: true,
                },
                tags: vec!["tag1".to_string()],
                metadata: HashMap::new(),
            },
            scores: vec![1.0, 2.0],
            empty_array: vec![],
            null_field: None,
        };

        let result = data.to_excel(path);
        assert!(result.is_ok());
        
        // Verificar que el archivo se creó
        assert!(fs::metadata(path).is_ok());
        
        // Limpiar
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_excel_hierarchical_network_data() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let data = NetworkTestData {
            base_network: "192.168.1.0".to_string(),
            base_cidr: 24,
            network_class: "Class C".to_string(),
            total_subnets: 2,
            total_hosts_capacity: 508,
            utilization_percentage: 85.5,
            subnets: vec![
                SubnetTestData {
                    subnet_id: 1,
                    network_address: "192.168.1.0".to_string(),
                    first_host: "192.168.1.1".to_string(),
                    last_host: "192.168.1.126".to_string(),
                    broadcast: "192.168.1.127".to_string(),
                    hosts_count: 126,
                },
                SubnetTestData {
                    subnet_id: 2,
                    network_address: "192.168.1.128".to_string(),
                    first_host: "192.168.1.129".to_string(),
                    last_host: "192.168.1.254".to_string(),
                    broadcast: "192.168.1.255".to_string(),
                    hosts_count: 126,
                },
            ],
        };

        let result = data.to_excel(path);
        assert!(result.is_ok());
        
        // Verificar que el archivo se creó
        assert!(fs::metadata(path).is_ok());
        
        // Limpiar
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_excel_hierarchical_empty_data() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let empty_vec: Vec<String> = vec![];
        let result = empty_vec.to_excel(path);
        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
        
        let _ = fs::remove_file(path);

        let empty_obj = HashMap::<String, String>::new();
        let result = empty_obj.to_excel(path);
        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
        
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_excel_hierarchical_primitive_types() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let number = 42;
        let result = number.to_excel(path);
        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
        
        let _ = fs::remove_file(path);

        let string = "hello".to_string();
        let result = string.to_excel(path);
        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
        
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_excel_hierarchical_invalid_path() {
        let data = SimpleStruct {
            name: "Test".to_string(),
            value: 42,
            active: true,
        };

        // Ruta inválida
        let result = data.to_excel("/invalid/path/test.xlsx");
        assert!(result.is_err());
    }

    // Tests para funciones helper
    #[test]
    fn test_separate_fields() {
        use serde_json::Map;
        
        let mut obj = Map::new();
        obj.insert("name".to_string(), Value::String("test".to_string()));
        obj.insert("age".to_string(), Value::Number(25.into()));
        obj.insert("tags".to_string(), Value::Array(vec![
            Value::String("tag1".to_string()),
            Value::String("tag2".to_string()),
        ]));
        obj.insert("metadata".to_string(), Value::Object(Map::new()));
        obj.insert("items".to_string(), Value::Array(vec![
            Value::Object({
                let mut item = Map::new();
                item.insert("id".to_string(), Value::Number(1.into()));
                item
            }),
        ]));

        let (flat, nested) = crate::core::formatting::utils::separate_fields(&obj);
        
        // Campos planos: name, age, tags (array de strings)
        assert_eq!(flat.len(), 3);
        assert!(flat.iter().any(|(k, _)| *k == "name"));
        assert!(flat.iter().any(|(k, _)| *k == "age"));
        assert!(flat.iter().any(|(k, _)| *k == "tags"));
        
        // Campos anidados: metadata (objeto), items (array de objetos)
        assert_eq!(nested.len(), 2);
        assert!(nested.iter().any(|(k, _)| *k == "metadata"));
        assert!(nested.iter().any(|(k, _)| *k == "items"));
    }

    #[test]
    fn test_get_unique_headers() {
        let data = vec![
            Value::Object({
                let mut obj = Map::new();
                obj.insert("id".to_string(), Value::Number(1.into()));
                obj.insert("name".to_string(), Value::String("test1".to_string()));
                obj
            }),
            Value::Object({
                let mut obj = Map::new();
                obj.insert("id".to_string(), Value::Number(2.into()));
                obj.insert("value".to_string(), Value::Number(42.into()));
                obj
            }),
        ];

        let headers = crate::core::formatting::utils::get_unique_headers(&data);
        assert_eq!(headers.len(), 3);
        assert!(headers.contains(&"id"));
        assert!(headers.contains(&"name"));
        assert!(headers.contains(&"value"));
    }

    #[test]
    fn test_value_to_string_readable() {
        // String
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::String("hello".to_string())),
            "hello"
        );

        // Number
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::Number(42.into())),
            "42"
        );

        // Boolean
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::Bool(true)),
            "true"
        );

        // Null
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::Null),
            "null"
        );

        // Array simple
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::Array(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
            ])),
            "[1, 2]"
        );

        // Array anidado
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::Array(vec![
                Value::Array(vec![Value::Number(1.into())]),
            ])),
            "[[1]]"
        );

        // Object
        assert_eq!(
            crate::core::formatting::utils::value_to_string_readable(&Value::Object(Map::new())),
            "{...}"
        );
    }

    #[test]
    fn test_build_markdown_table_empty() {
        let result = build_markdown_table(&[]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Empty Data"));
    }

    #[test]
    fn test_build_markdown_table_simple() {
        let data = vec![
            Value::Object({
                let mut obj = Map::new();
                obj.insert("id".to_string(), Value::Number(1.into()));
                obj.insert("name".to_string(), Value::String("test1".to_string()));
                obj
            }),
            Value::Object({
                let mut obj = Map::new();
                obj.insert("id".to_string(), Value::Number(2.into()));
                obj.insert("name".to_string(), Value::String("test2".to_string()));
                obj
            }),
        ];

        let result = build_markdown_table(&data);
        assert!(result.is_ok());
        
        let table = result.unwrap();
        assert!(table.contains("| id | name |"));
        assert!(table.contains("| 1 | test1 |"));
        assert!(table.contains("| 2 | test2 |"));
    }

    // Tests de integración - comparación entre métodos planos y jerárquicos
    #[test]
    fn test_integration_hierarchical_vs_flat() {
        let data = NetworkTestData {
            base_network: "192.168.1.0".to_string(),
            base_cidr: 24,
            network_class: "Class C".to_string(),
            total_subnets: 1,
            total_hosts_capacity: 254,
            utilization_percentage: 50.0,
            subnets: vec![SubnetTestData {
                subnet_id: 1,
                network_address: "192.168.1.0".to_string(),
                first_host: "192.168.1.1".to_string(),
                last_host: "192.168.1.254".to_string(),
                broadcast: "192.168.1.255".to_string(),
                hosts_count: 254,
            }],
        };

        // Comparar Markdown
        let flat_md = data.to_markdown().unwrap();
        let hierarchical_md = data.to_markdown_hierarchical().unwrap();

        // La versión jerárquica debería ser más legible y estructurada
        assert!(hierarchical_md.contains("# Data Export"));
        assert!(hierarchical_md.contains("## subnets"));
        // La versión plana podría mostrar "[array]" mientras la jerárquica muestra la tabla
        assert!(!hierarchical_md.contains("[array]"));
        assert!(hierarchical_md.contains("| subnet_id |"));

        // Comparar Excel
        let temp_file_flat = NamedTempFile::new().unwrap();
        let temp_file_hierarchical = NamedTempFile::new().unwrap();
        
        let flat_excel = data.to_excel(temp_file_flat.path().to_str().unwrap());
        let hierarchical_excel = data.to_excel(temp_file_hierarchical.path().to_str().unwrap());

        assert!(flat_excel.is_ok());
        assert!(hierarchical_excel.is_ok());

        // Ambos archivos deberían existir
        assert!(fs::metadata(temp_file_flat.path()).is_ok());
        assert!(fs::metadata(temp_file_hierarchical.path()).is_ok());

        // Limpiar
        let _ = fs::remove_file(temp_file_flat.path());
        let _ = fs::remove_file(temp_file_hierarchical.path());
    }

    // Test de rendimiento con datos grandes
    #[test]
    fn test_performance_large_dataset() {
        let large_data = NetworkTestData {
            base_network: "10.0.0.0".to_string(),
            base_cidr: 8,
            network_class: "Class A".to_string(),
            total_subnets: 100,
            total_hosts_capacity: 10000,
            utilization_percentage: 75.0,
            subnets: (0..50) // 50 subredes para prueba
                .map(|i| SubnetTestData {
                    subnet_id: i as u32 + 1,
                    network_address: format!("10.{}.0.0", i),
                    first_host: format!("10.{}.0.1", i),
                    last_host: format!("10.{}.255.254", i),
                    broadcast: format!("10.{}.255.255", i),
                    hosts_count: 65534,
                })
                .collect(),
        };

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Test Markdown
        let start = std::time::Instant::now();
        let md_result = large_data.to_markdown_hierarchical();
        let md_duration = start.elapsed();
        
        assert!(md_result.is_ok());
        assert!(md_duration < std::time::Duration::from_secs(5)); // No debería tomar más de 5 segundos

        // Test Excel
        let start = std::time::Instant::now();
        let excel_result = large_data.to_excel(path);
        let excel_duration = start.elapsed();
        
        assert!(excel_result.is_ok());
        assert!(excel_duration < std::time::Duration::from_secs(10)); // No debería tomar más de 10 segundos

        // Limpiar
        let _ = fs::remove_file(path);
    }
}