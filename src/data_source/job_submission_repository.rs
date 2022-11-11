use std::{error::Error, fs, path::PathBuf, sync::Arc};

use crate::domain::{input::Input, job_submission::JobSubmission};

pub struct JobSubmissionRepository {
    jobs_file_path: PathBuf,
    jobs: Vec<Arc<JobSubmission>>,
}

impl JobSubmissionRepository {
    pub fn new(jobs_file_path: PathBuf, jobs: Vec<Arc<JobSubmission>>) -> Self {
        JobSubmissionRepository {
            jobs_file_path: jobs_file_path.clone(),
            jobs: jobs,
        }
    }

    pub fn insert(&mut self, job_sub: Arc<JobSubmission>) -> Result<(), Box<dyn Error>> {
        self.jobs.push(job_sub);

        Ok(())
    }

    pub fn get_new_job_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        let records = fs::read_dir(&self.jobs_file_path)?;

        let mut max_num = 0;
        for record in records {
            let dir = record?;
            if let Some(file_name) = dir.file_name().to_str() {
                if let Ok(num) = file_name.parse() {
                    max_num = std::cmp::max(num, max_num);
                }
            }
        }

        Ok(self.jobs_file_path.clone().join((max_num + 1).to_string()))
    }

    pub fn load(path: PathBuf) -> Result<Vec<Arc<JobSubmission>>, Box<dyn Error>> {
        let file = fs::File::open(path.as_path())?;
        let jobs: Vec<Arc<JobSubmission>> = serde_yaml::from_reader(file)?;

        Ok(jobs)
    }

    pub fn findAll(&mut self) -> Result<Vec<Arc<JobSubmission>>, Box<dyn Error>> {
        Ok(self.jobs.clone())
    }

    pub fn flush(&self) -> Result<(), Box<dyn Error>> {
        let writer = fs::File::open(self.jobs_file_path.as_path())?;

        serde_yaml::to_writer(writer, &self.jobs);

        Ok(())
    }
}
