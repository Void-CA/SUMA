use crate::core::formatting::export::Exporter;

// Implementación para JSON
pub struct JsonExporter {
    output: String,
    stack: Vec<char>,
    first_in_container: bool,
}

impl JsonExporter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            stack: Vec::new(),
            first_in_container: true,
        }
    }

    fn current_container(&self) -> Option<char> {
        self.stack.last().copied()
    }

    fn write_comma_if_needed(&mut self) {
        if !self.first_in_container {
            self.output.push(',');
        }
        self.first_in_container = false;
    }
}

impl Exporter for JsonExporter {
    fn begin(&mut self) {
        self.output.push('{');
        self.stack.push('{');
        self.first_in_container = true;
    }

    fn write_field(&mut self, key: &str, value: &str) {
        self.write_comma_if_needed();
        self.output.push_str(&format!("\"{}\":\"{}\"", key, value));
    }

    fn begin_object(&mut self, key: &str) {
        self.write_comma_if_needed();
        
        if !key.is_empty() {
            // Para objetos con key: "key": {
            self.output.push_str(&format!("\"{}\":{{", key));
        } else {
            // Para objetos sin key (en arrays): {
            self.output.push('{');
        }
        
        self.stack.push('{');
        self.first_in_container = true;
    }

    fn end_object(&mut self) {
        if let Some('{') = self.stack.pop() {
            self.output.push('}');
        }
        self.first_in_container = false;
    }

    fn begin_array(&mut self, key: &str) {
        self.write_comma_if_needed();
        self.output.push_str(&format!("\"{}\":[", key));
        self.stack.push('[');
        self.first_in_container = true;
    }

    fn write_array_item(&mut self, value: &str) {
        self.write_comma_if_needed();
        self.output.push_str(&format!("\"{}\"", value));
    }

    fn end_array(&mut self) {
        if let Some('[') = self.stack.pop() {
            self.output.push(']');
        }
        self.first_in_container = false;
    }

    fn end(&mut self) {
        while let Some(container) = self.stack.pop() {
            match container {
                '{' => self.output.push('}'),
                '[' => self.output.push(']'),
                _ => {}
            }
        }
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}