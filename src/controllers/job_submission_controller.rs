use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use mockall_double::double;

#[double]
use crate::domain::input::Input;

use crate::{
    app::App,
    data_source::{
        job_submission_repository::JobSubmissionRepository, program_repository::FindError,
    },
    domain::{
        domain_object::DomainObject,
        job_submission::{JobSubmission, JobSubmissionId},
    },
};

pub struct JobSubissionController {
    app: Rc<App>,
}

impl JobSubissionController {
    pub fn new(app: Rc<App>) -> Self {
        JobSubissionController { app: app }
    }

    fn get_new_job_dir(&self) -> Option<PathBuf> {
        let job_sub_repo = self.app.get_job_submission_repository();

        Some(job_sub_repo.as_ref()?.get_new_job_dir().ok()?)
    }

    pub fn create_job(
        &self,
        inputs: Vec<&Path>,
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

        let program_repository = self.app.get_program_repository();

        let inputs = inputs
            .iter()
            .map(|p| Input::new(p.to_path_buf()).unwrap())
            .collect();

        let program = program_repository.find(program)?;

        let job = JobSubmission::new(program, inputs, job_location, parameters);

        job.insert().map_err(|_| FindError)?;

        return Ok(job.get_location());
    }

    pub fn run_job() -> i32 {
        1
    }
}
