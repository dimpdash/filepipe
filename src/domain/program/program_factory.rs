use std::collections::HashMap;

use super::Program;

use crate::domain::program::non_interactive_program::NonInteractiveProgram;

pub struct ProgramFactory{}

impl ProgramFactory {
    pub fn create(program : &str, parameters : HashMap<String, String>) -> Box<dyn Program> {
        
        Box::new(NonInteractiveProgram{})
    }
}