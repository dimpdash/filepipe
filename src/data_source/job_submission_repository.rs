use std::{error::Error, fs, path::PathBuf};

pub struct JobSubmissionRepository {
    default_job_dir: PathBuf,
}

impl JobSubmissionRepository {
    pub fn new(default_job_dir: PathBuf) -> Self {
        JobSubmissionRepository { default_job_dir }
    }

    pub fn get_new_job_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        let records = fs::read_dir(&self.default_job_dir)?;

        let mut max_num = 0;
        for record in records {
            let dir = record?;
            if let Some(file_name) = dir.file_name().to_str() {
                if let Ok(num) = file_name.parse() {
                    max_num = std::cmp::max(num, max_num);
                }
            }
        }

        Ok(self.default_job_dir.clone().join((max_num + 1).to_string()))
    }
}
