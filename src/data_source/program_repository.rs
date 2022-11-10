use std::{ffi::OsStr, fs, path::PathBuf, rc::Rc};

use crate::domain::program::{non_interactive_program::NonInteractiveProgram, Program};

#[derive(Debug)]
pub struct FindError;

pub struct ProgramRepository {
    home: PathBuf,
}

impl ProgramRepository {
    pub fn new(home: PathBuf) -> Self {
        ProgramRepository { home }
    }

    pub fn find(&self, id: &str) -> Result<Rc<dyn Program>, FindError> {
        let entites = fs::read_dir(&self.home).map_err(|_| FindError)?;

        for e in entites {
            if let Ok(d) = e {
                let p = d.path();
                if p.file_name() == Some(OsStr::new(id)) {
                    return Ok(Rc::new(NonInteractiveProgram::new(p)));
                }
            }
        }

        return Err(FindError);
    }
}
