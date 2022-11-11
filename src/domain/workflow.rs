use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::job_submission::JobSubmission;

#[derive(Serialize, Deserialize)]
pub struct Workflow {
    job_submissions: Vec<JobSubmission>,
}

impl Workflow {
    pub fn new() -> Workflow {
        let workflow_path = PathBuf::from("./jobs.yaml");
        let rdr = fs::File::open(workflow_path).unwrap();
        let w: Workflow = serde_yaml::from_reader(rdr).unwrap();

        return w;
    }
}
