use std::{collections::HashMap, ffi::OsStr, fs, path::PathBuf, rc::Rc, sync::Arc};

use crate::domain::program::{non_interactive_program::NonInteractiveProgram, Program};

#[derive(Debug)]
pub struct FindError;

pub struct ProgramRepository {
    home: PathBuf,
    programs: HashMap<String, Arc<dyn Program>>,
}

impl ProgramRepository {
    pub fn new(home: PathBuf) -> Self {
        ProgramRepository {
            home,
            programs: HashMap::new(),
        }
    }

    pub fn find(&self, id: &str) -> Result<Arc<dyn Program>, FindError> {
        Ok(self.programs.get(id).ok_or(FindError)?.clone())
    }

    pub fn insert(&mut self, program: Box<dyn Program>) {
        let _ = self.programs.insert(program.get_name(), Arc::from(program));
    }
}
