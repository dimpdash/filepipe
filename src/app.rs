use crate::data_source::{
    job_submission_repository::JobSubmissionRepository, program_repository::ProgramRepository,
};

pub struct App {
    program_repository: ProgramRepository,
    job_submission_repository: Option<JobSubmissionRepository>,
}

impl App {
    pub fn new(
        program_repository: ProgramRepository,
        job_submission_repository: Option<JobSubmissionRepository>,
    ) -> Self {
        App {
            program_repository,
            job_submission_repository,
        }
    }

    pub fn get_program_repository(&self) -> &ProgramRepository {
        &self.program_repository
    }

    pub fn get_job_submission_repository(&self) -> &Option<JobSubmissionRepository> {
        &self.job_submission_repository
    }
}
