use crate::domain::program::Program;
use crate::domain::job_submission::job_submission_state::JobSubmissionState;

use crate::domain::job_submission::job_submission_state::dead_state::DeadState;

pub mod job_submission_state;

pub struct JobSubmission {
    state : Box<dyn JobSubmissionState>,
    program : Box<dyn Program>
}

impl JobSubmission {
    pub fn new(program : Box<dyn Program>) -> JobSubmission {
        let state = Box::new(DeadState{});

        JobSubmission {
            state : state,
            program : program,
        }
    }
}

pub type JobSubmissionId = String;