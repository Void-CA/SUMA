use crate::core::formatting::export::Exporter;


// Implementación para XML
pub struct XmlExporter {
    output: String,
    stack: Vec<String>,
}

impl XmlExporter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            stack: Vec::new(),
        }
    }
}

impl Exporter for XmlExporter {
    fn begin(&mut self) {
        self.output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>");
        self.stack.push("root".to_string());
    }

    fn write_field(&mut self, key: &str, value: &str) {
        let sanitized_key = key.replace(' ', "_");
        self.output.push_str(&format!("\n  <{}>{}</{}>", sanitized_key, value, sanitized_key));
    }

    fn begin_object(&mut self, key: &str) {
        let sanitized_key = key.replace(' ', "_");
        self.output.push_str(&format!("\n  <{}>", sanitized_key));
        self.stack.push(sanitized_key);
    }

    fn end_object(&mut self) {
        if let Some(key) = self.stack.pop() {
            self.output.push_str(&format!("\n  </{}>", key));
        }
    }

    fn begin_array(&mut self, key: &str) {
        let sanitized_key = key.replace(' ', "_");
        self.output.push_str(&format!("\n  <{}>", sanitized_key));
        self.stack.push(sanitized_key);
    }

    fn write_array_item(&mut self, value: &str) {
        self.output.push_str(&format!("\n    <item>{}</item>", value));
    }

    fn end_array(&mut self) {
        if let Some(key) = self.stack.pop() {
            self.output.push_str(&format!("\n  </{}>", key));
        }
    }

    fn end(&mut self) {
        while let Some(_) = self.stack.pop() {
            self.output.push_str("</root>");
        }
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}
