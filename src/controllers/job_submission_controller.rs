use std::{collections::HashMap, path::{Path}};

use mockall_double::double;

#[double]
use crate::domain::input::Input;

use crate::{domain::{job_submission::{JobSubmissionId, JobSubmission}, domain_object::DomainObject}, data_source::program_repository::{ProgramRepository, FindError}};

pub struct JobSubissionControler;
impl JobSubissionControler {
    pub fn create_job(inputs: Vec<&Path>, program : &str, parameters : HashMap<String, String>) -> Result<JobSubmissionId, FindError> {
        let home = Path::new("./example/programs");
        let job_location = Path::new("./1");
        let program_repository = ProgramRepository::new(home.into());

        let inputs = inputs.iter().map(|p| Input::new(p.to_path_buf()).unwrap()).collect();

        let program = program_repository.find(program)?;

        let job = JobSubmission::new(program, inputs, job_location.into(), parameters);

        job.insert().map_err(|_| FindError)?;
    
        return Ok("h".to_string());
    }
    
    pub fn run_job() -> i32 {
        1
    }
}