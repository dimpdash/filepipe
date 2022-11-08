use std::collections::{HashMap, BTreeMap};
use std::fs::File;
use std::hash::Hash;
use std::path::{PathBuf};
use std::error::Error;

use mockall_double::double;

use crate::domain::program::Program;
use crate::domain::job_submission::job_submission_state::JobSubmissionState;

use crate::domain::job_submission::job_submission_state::dead_state::DeadState;

use self::job_config::JobConfig;

use super::domain_object::DomainObject;



#[double]
use super::input::Input;

pub mod job_submission_state;
pub mod job_config;

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

pub type JobSubmissionId = String;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path(){
        use crate::domain::program::{MockProgram, self};

        let mut input = Input::default();
        input.expect_add_link()
            .returning(|_| Ok(()))
            .times(1);
        
        let mut input2 = Input::default();
            input2.expect_add_link()
                .returning(|_| Ok(()))
                .times(1);

        let inputs = vec![input, input2];
        let program = Box::new(MockProgram::new());
        let location = PathBuf::from(".");
        let parameters = HashMap::new();

        
        let j = JobSubmission::new(program, inputs, location, parameters);
        j.insert().unwrap();


    }

}

