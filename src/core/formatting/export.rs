pub trait Exporter {
    fn begin(&mut self);
    fn write_field(&mut self, key: &str, value: &str);
    fn begin_object(&mut self, key: &str);
    fn end_object(&mut self);
    fn begin_array(&mut self, key: &str);
    fn write_array_item(&mut self, value: &str);

    fn end_array(&mut self);
    fn end(&mut self);
    fn output(&self) -> String;
}

pub trait Exportable {
    fn export_with(&self, exporter: &mut dyn Exporter);
}

/// Representa una fila de datos tabulares.
pub trait TableRow {
    /// Devuelve los encabezados de la fila (una sola vez por tipo).
    fn headers() -> Vec<&'static str> where Self: Sized;
    /// Devuelve los valores de la fila, en el mismo orden que los encabezados.
    fn values(&self) -> Vec<String>;
}

pub trait CsvRow: TableRow {
    fn to_csv_row(&self) -> String {
        self.values().join(",")
    }
}

pub trait MarkdownRow: TableRow {
    fn to_markdown_row(&self) -> String {
        format!("| {} |", self.values().join(" | "))
    }

    fn markdown_header() -> String where Self: Sized {
        let headers = Self::headers().join(" | ");
        let separator = Self::headers().iter().map(|_| "---").collect::<Vec<_>>().join(" | ");
        format!("| {} |\n| {} |", headers, separator)
    }
}

pub trait XmlRow: TableRow {
    fn to_xml_row(&self) -> String where Self: Sized {
        let fields = Self::headers()
            .into_iter()
            .zip(self.values())
            .map(|(k, v)| format!("<{k}>{v}</{k}>"))
            .collect::<Vec<_>>()
            .join("");
        format!("<row>{}</row>", fields)
    }
}
