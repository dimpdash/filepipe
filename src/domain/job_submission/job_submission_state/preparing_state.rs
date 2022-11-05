use crate::domain::job_submission::JobSubmission;
use super::JobSubmissionState;

pub struct PreparingState {
    job_sub : Box<JobSubmission>
}

impl PreparingState {
    pub fn new(job_sub : Box<JobSubmission>) -> Self {
        PreparingState{ job_sub: job_sub }
    }
}

impl JobSubmissionState for PreparingState {
    fn run(&self) {

    }
}