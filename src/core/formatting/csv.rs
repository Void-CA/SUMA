use crate::core::formatting::export::Exporter;

// Implementación para CSV
pub struct CsvExporter {
    output: String,
    first: bool,
}

impl CsvExporter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            first: true,
        }
    }
}

impl Exporter for CsvExporter {
    fn begin(&mut self) {
        // CSV típicamente no necesita un begin especial
    }

    fn write_field(&mut self, key: &str, value: &str) {
        if self.first {
            self.output.push_str("key,value\n");
            self.first = false;
        }
        self.output.push_str(&format!("\"{}\",\"{}\"\n", key, value));
    }

    fn begin_object(&mut self, _key: &str) {
        // CSV no soporta objetos anidados de forma nativa
    }

    fn end_object(&mut self) {
        // CSV no soporta objetos anidados
    }

    fn begin_array(&mut self, key: &str) {
        self.output.push_str(&format!("\"{}\",\"[", key));
    }

    fn write_array_item(&mut self, value: &str) {
        self.output.push_str(&format!("{};", value));
    }

    fn end_array(&mut self) {
        if self.output.ends_with(';') {
            self.output.pop(); // Remover el último ';'
        }
        self.output.push_str("]\"\n");
    }

    fn end(&mut self) {
        // CSV no necesita un end especial
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}