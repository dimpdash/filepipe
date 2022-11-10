use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::Lazy;


use crate::data_source::{
    job_submission_repository::JobSubmissionRepository,
};

use crate::domain::programs::Programs;

pub struct App {
    programs: Programs,
    job_submission_repository: JobSubmissionRepository,
}

pub static APP: Lazy<Mutex<App>> = Lazy::new(|| {
    Mutex::new(App::new(
        Programs::new(),
        JobSubmissionRepository::new(PathBuf::from("./jobs")),
    ))
});

impl App {
    pub fn new(programs: Programs, job_submission_repository: JobSubmissionRepository) -> Self {
        App {
            programs,
            job_submission_repository,
        }
    }

    pub fn get_programs(&mut self) -> &mut Programs {
        &mut self.programs
    }

    // pub fn new(
    //     program_repository: ProgramRepository,
    //     job_submission_repository: Option<JobSubmissionRepository>,
    // ) -> Self {
    //     App {
    //         program_repository,
    //         job_submission_repository,
    //     }
    // }

    // pub fn get_program_repository(&self) -> &ProgramRepository {
    //     &self.program_repository
    // }

    pub fn get_job_submission_repository(&self) -> &JobSubmissionRepository {
        &self.job_submission_repository
    }
}
