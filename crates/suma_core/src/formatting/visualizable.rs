use crate::formatting::error::ExportError;

/// Exporta a formato DOT (Graphviz)
pub trait ToDot {
    fn to_dot(&self) -> Result<String, ExportError>;
}

/// Exporta a formato Mermaid
pub trait ToMermaid {
    fn to_mermaid(&self) -> Result<String, ExportError>;
}

/// Exporta a formato PlantUML
pub trait ToPlantUml {
    fn to_plantuml(&self) -> Result<String, ExportError>;
}
