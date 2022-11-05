use std::{fs::File, collections::HashMap};
use crate::domain::{job_submission::{JobSubmissionId, JobSubmission}};
use crate::domain::program::program_factory::ProgramFactory;

pub struct JobSubissionControler;
impl JobSubissionControler {
    pub fn create_job(inputs: File, program : &str, parameters : HashMap<String, String>) -> JobSubmissionId {
        let program = ProgramFactory::create(program, parameters);
        let job = JobSubmission::new(program);
    

        "h".to_string()
    }
    pub fn run_job() -> i32 {
        1
    }
}