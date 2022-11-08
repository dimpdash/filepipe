use std::path::PathBuf;

use super::Program;


pub struct NonInteractiveProgram {
    dir : PathBuf
}

impl NonInteractiveProgram {
    pub fn new(dir : PathBuf) -> Self {
        NonInteractiveProgram{dir : dir}
    }
}

impl Program for NonInteractiveProgram {
    fn get_name(&self) -> String {
        self.dir.to_str().unwrap().to_string()
    }
}