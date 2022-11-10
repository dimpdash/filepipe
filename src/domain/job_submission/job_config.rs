use std::{collections::HashMap, error::Error, fs::File, path::PathBuf};


use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JobConfig {
    pub parameters: HashMap<String, String>,
    pub program: String,
}

impl JobConfig {
    pub fn add_file(&self, at: &PathBuf) -> Result<(), Box<dyn Error>> {
        let config_file = File::create(at.clone().join("filepipe-config.yaml"))?;

        serde_yaml::to_writer(config_file, &self)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        fs::{remove_file, File},
        path::PathBuf,
    };

    use super::JobConfig;

    #[test]
    fn create_config() {
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert("name".to_string(), "test name".to_string());
        m.insert("iterations".to_string(), "1000".to_string());

        let job_config = JobConfig {
            parameters: m.clone(),
            program: "./program_location".to_string(),
        };

        let location = PathBuf::from("./tests/artefacts/generated/job_config/create_config");
        let yaml_location = location.join("filepipe-config.yaml");
        let _ = remove_file(&yaml_location);

        job_config.add_file(&location).unwrap();

        let rdr = File::open(&yaml_location).unwrap();
        let f: JobConfig = serde_yaml::from_reader(rdr).unwrap();

        assert_eq!(&f.parameters, &m);
        assert_eq!(f.program, "./program_location".to_string());

        let _ = remove_file(&yaml_location);
    }
}
