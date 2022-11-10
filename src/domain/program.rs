use mockall::automock;
use serde::{Deserialize, Serialize};

pub mod non_interactive_program;

#[typetag::serde(tag = "type")]
pub trait Program: Send + Sync {
    fn get_name(&self) -> String;
}
