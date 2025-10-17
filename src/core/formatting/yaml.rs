use crate::core::formatting::export::Exporter;

// Implementación para YAML
pub struct YamlExporter {
    output: String,
    indent_level: usize,
    first_in_container: bool,
}

impl YamlExporter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            first_in_container: true,
        }
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }
}

impl Exporter for YamlExporter {
    fn begin(&mut self) {
        // YAML no necesita begin especial
    }

    fn write_field(&mut self, key: &str, value: &str) {
        self.output.push_str(&format!("{}{}: {}\n", self.indent(), key, value));
    }

    fn begin_object(&mut self, key: &str) {
        self.output.push_str(&format!("{}{}:\n", self.indent(), key));
        self.indent_level += 1;
    }

    fn end_object(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    fn begin_array(&mut self, key: &str) {
        self.output.push_str(&format!("{}{}:\n", self.indent(), key));
        self.indent_level += 1;
    }

    fn write_array_item(&mut self, value: &str) {
        self.output.push_str(&format!("{}- {}\n", self.indent(), value));
    }

    fn end_array(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    fn end(&mut self) {
        // YAML no necesita end especial
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}