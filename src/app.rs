use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::data_source::job_submission_repository::JobSubmissionRepository;

use crate::data_source::program_repository::ProgramRepository;

pub struct App {
    program_repo: ProgramRepository,
    job_submission_repo: JobSubmissionRepository,
}

pub static APP: Lazy<Mutex<App>> = Lazy::new(|| {
    let jobs_file_path = PathBuf::from("./jobs");
    let jobs = JobSubmissionRepository::load(jobs_file_path.clone());

    let jobs = match jobs {
        Ok(j) => j,
        Err(_) => vec![],
    };

    Mutex::new(App::new(
        ProgramRepository::new(PathBuf::from("./programs")),
        JobSubmissionRepository::new(jobs_file_path, jobs),
    ))
});

impl App {
    pub fn new(
        program_repo: ProgramRepository,
        job_submission_repo: JobSubmissionRepository,
    ) -> Self {
        App {
            program_repo,
            job_submission_repo,
        }
    }

    pub fn get_program_repo(&mut self) -> &mut ProgramRepository {
        &mut self.program_repo
    }

    pub fn get_job_submission_repo(&mut self) -> &mut JobSubmissionRepository {
        &mut self.job_submission_repo
    }
}
