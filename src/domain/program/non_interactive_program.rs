use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Program;

#[derive(Serialize, Deserialize)]
pub struct NonInteractiveProgram {
    dir: PathBuf,
}

impl NonInteractiveProgram {
    pub fn new(dir: PathBuf) -> Self {
        NonInteractiveProgram { dir: dir }
    }
}

#[typetag::serde]
impl Program for NonInteractiveProgram {
    fn get_name(&self) -> String {
        self.dir.to_str().unwrap().to_string()
    }
}
