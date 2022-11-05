use std::collections::HashMap;
use std::path::{PathBuf};
use std::error::Error;

use crate::domain::program::Program;
use crate::domain::job_submission::job_submission_state::JobSubmissionState;

use crate::domain::job_submission::job_submission_state::dead_state::DeadState;

use super::domain_object::DomainObject;
use super::input::Input;

pub mod job_submission_state;

pub struct JobSubmission {
    state : Box<dyn JobSubmissionState>,
    program : Box<dyn Program>,
    inputs : Vec<Input>,
    location : PathBuf,
    parameters : HashMap<String, String>
}

impl JobSubmission {
    pub fn new(program : Box<dyn Program>, inputs : Vec<Input>, location : PathBuf, parameters : HashMap<String, String>) -> Self {
        let state = Box::new(DeadState{});

        JobSubmission {
            state,
            program,
            inputs,
            location,
            parameters
        }
    }
}

impl DomainObject for JobSubmission {
    fn insert(&self) -> Result<(), Box<dyn Error>> {
        
        //create folder
        std::fs::create_dir_all(&self.location).map_err(|e| Box::new(e))?;
        
        //create input links
        for input in self.inputs.iter() {
            input.add_link(&self.location).map_err(|e| Box::new(e))?
        }

        //create parameter file




        Ok(())
    }

    fn update(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn delete(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub type JobSubmissionId = String;