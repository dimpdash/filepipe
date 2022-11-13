use serde::{Deserialize, Serialize};

pub type Transformers = Vec<Transformer>;

#[derive(Serialize, Deserialize)]
pub struct Transformer {
    pub name: String,
    pub executor_path: String,
    pub valid_dependencies: Option<DependencyExtensions>,
    pub typ: TransformerTypes,
}

pub type DependencyExtensions = Vec<String>;

#[derive(Serialize, Deserialize)]
pub enum TransformerTypes {
    NonInteractiveTransformer,
}
