use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};



use crate::{
    app::APP,
    domain::input::{Input},
};

use crate::{
    data_source::{
        program_repository::FindError,
    },
    domain::{
        domain_object::DomainObject,
        job_submission::{JobSubmission, JobSubmissionId},
    },
};

pub struct JobSubissionController {}

impl JobSubissionController {
    pub fn new() -> Self {
        JobSubissionController {}
    }

    fn get_new_job_dir(&self) -> Option<PathBuf> {
        let app = APP.lock().unwrap();
        let job_sub_repo = app.get_job_submission_repository();

        Some(job_sub_repo.get_new_job_dir().ok()?)
    }

    pub fn create_job(
        &self,
        _inputs: Vec<&Path>,
        program: &str,
        parameters: HashMap<String, String>,
        job_location: Option<PathBuf>,
    ) -> Result<JobSubmissionId, FindError> {
        let job_location = if !job_location.is_none() {
            job_location
        } else {
            Some(self.get_new_job_dir().ok_or(FindError)?)
        };

        let job_location = job_location.unwrap();

        let mut app = APP.lock().unwrap();
        let programs = app.get_programs();

        // let inputs = inputs
        //     .iter()
        //     .map(|p| Input::new(p.to_path_buf()).unwrap())
        //     .collect();
        let inputs: Vec<Box<dyn Input>> = vec![];

        let program = programs.get_program(program).unwrap();

        let job = JobSubmission::new(program, inputs, job_location, parameters);

        job.insert().map_err(|_| FindError)?;

        return Ok(job.get_location());
    }

    pub fn run_job() -> i32 {
        1
    }
}
