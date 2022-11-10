use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use mockall_double::double;
use serde::{Deserialize, Serialize};

use crate::domain::job_submission::job_submission_state::JobSubmissionState;
use crate::domain::program::Program;

use crate::domain::job_submission::job_submission_state::dead_state::DeadState;

use crate::domain::input::Input;

use self::job_config::JobConfig;

use super::domain_object::DomainObject;

pub mod job_config;
pub mod job_submission_state;

pub struct JobSubmission {
    program: Arc<dyn Program>,
    inputs: Vec<Box<dyn Input>>,
    location: PathBuf,
    parameters: HashMap<String, String>,
}

impl JobSubmission {
    pub fn new(
        program: Arc<dyn Program>,
        inputs: Vec<Box<dyn Input>>,
        location: PathBuf,
        parameters: HashMap<String, String>,
    ) -> Self {
        let state = Box::new(DeadState {});

        JobSubmission {
            program,
            inputs,
            location,
            parameters,
        }
    }
}

impl JobSubmission {
    pub fn get_location(&self) -> PathBuf {
        self.location.clone()
    }
}

impl DomainObject for JobSubmission {
    fn insert(&self) -> Result<(), Box<dyn Error>> {
        //create folder
        std::fs::create_dir_all(&self.location).map_err(|e| Box::new(e))?;

        //create input links
        for input in self.inputs.iter() {
            // input.add_link(&self.location).map_err(|e| Box::new(e))?
        }

        //create parameter file
        let config = JobConfig {
            parameters: self.parameters.clone(),
            program: self.program.get_name(),
        };

        config.add_file(&self.location)?;

        Ok(())
    }

    fn update(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn delete(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub type JobSubmissionId = PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        // use crate::domain::program::MockProgram;

        // let mut input = Input::default();
        // input.expect_add_link().returning(|_| Ok(())).times(1);

        // let mut input2 = Input::default();
        // input2.expect_add_link().returning(|_| Ok(())).times(1);

        // let inputs = vec![input, input2];
        // let program = Box::new(MockProgram::new());
        // let location = PathBuf::from(".");
        // let parameters = HashMap::new();

        // let j = JobSubmission::new(program, inputs, location, parameters);
        // j.insert().unwrap();
    }
}
