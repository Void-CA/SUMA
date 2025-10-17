use crate::core::formatting::export::Exporter;

pub struct MarkdownExporter {
    output: String,
    stack: Vec<(String, bool)>, // (key, is_array)
}

impl MarkdownExporter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            stack: Vec::new(),
        }
    }

    fn indent(&self) -> String {
        "  ".repeat(self.stack.len())
    }
}

impl Exporter for MarkdownExporter {
    fn begin(&mut self) {
        self.output.push_str("# Export\n\n");
    }

    fn write_field(&mut self, key: &str, value: &str) {
        let indent = self.indent();
        if self.stack.is_empty() {
            self.output.push_str(&format!("- **{}**: {}\n", key, value));
        } else {
            // inside an object => use list-style key/value with indentation
            self.output.push_str(&format!("{}- **{}**: {}\n", indent, key, value));
        }
    }

    fn begin_object(&mut self, key: &str) {
        let indent = self.indent();
        self.output.push_str(&format!("{}### {}\n\n", indent, key));
        self.stack.push((key.to_string(), false));
    }

    fn end_object(&mut self) {
        if !self.stack.is_empty() {
            self.stack.pop();
            self.output.push('\n');
        }
    }

    fn begin_array(&mut self, key: &str) {
        let indent = self.indent();
        self.output.push_str(&format!("{}### {}\n\n", indent, key));
        self.stack.push((key.to_string(), true));
    }

    fn write_array_item(&mut self, value: &str) {
        let indent = self.indent();
        // array items are simple list items
        self.output.push_str(&format!("{}- {}\n", indent, value));
    }

    fn end_array(&mut self) {
        if !self.stack.is_empty() {
            self.stack.pop();
            self.output.push('\n');
        }
    }

    fn end(&mut self) {
        // close any remaining containers gently
        while !self.stack.is_empty() {
            self.stack.pop();
            self.output.push('\n');
        }
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}