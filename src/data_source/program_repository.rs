use std::{path::PathBuf, fs , ffi::OsStr};

use crate::domain::program::{Program, non_interactive_program::NonInteractiveProgram};

#[derive(Debug)]
pub struct FindError;

pub struct ProgramRepository {
    home: PathBuf
}

impl ProgramRepository {
    pub fn new(home : PathBuf) -> Self {
        ProgramRepository {
            home
        }
    }

    pub fn find(&self, id : &str) -> Result<Box<dyn Program>, FindError> {
        let entites = fs::read_dir(&self.home).map_err(|_| FindError)?;

        for e in entites {
            if let Ok(d) = e {
                let p = d.path();
                if p.file_name() == Some(OsStr::new(id)) {
                    return Ok(Box::new(NonInteractiveProgram::new(p)));
                }
            }
        }
        
        return Err(FindError);
    }
}