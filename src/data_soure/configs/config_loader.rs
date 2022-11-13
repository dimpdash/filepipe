use std::collections::HashMap;
use std::iter::Map;
use std::path::PathBuf;
use std::rc::Rc;

use regex::Regex;

use super::jobs;
use super::transformers;

use crate::models::file_specifier::regex_file_specifier::RegexFileSpecifier;
use crate::models::file_specifier::single_file_specifier::SingleFileSpecifier;
use crate::models::file_specifier::Dependencies;
use crate::models::file_specifier::Dependency;
use crate::models::file_specifier::FileSpecifier;
use crate::models::file_specifier::Target;
use crate::models::file_specifier::Targets;
use crate::models::job::Job;
use crate::models::transformer::{
    non_interactive_transformer::NonInteractiveTransformer, Transformer,
};

pub struct ConfigLoader {
    transformers: HashMap<String, Rc<dyn Transformer>>,
}

impl ConfigLoader {
    pub fn load(
        &mut self,
        jobs_config: jobs::Jobs,
        transformers: transformers::Transformers,
    ) -> (Vec<Job>, Vec<Rc<dyn Transformer>>) {
        self.transformers = self.load_transformers(&transformers);
        let jobs = self.load_jobs(&jobs_config);
        let transformers = self.transformers.values().cloned().collect();

        (jobs, transformers)
    }

    fn load_transformers(
        &mut self,
        transformers: &transformers::Transformers,
    ) -> HashMap<String, Rc<dyn Transformer>> {
        transformers
            .iter()
            .map(|t| (t.name.clone(), self.load_transformer(t)))
            .collect()
    }

    fn load_transformer(&mut self, transformer: &transformers::Transformer) -> Rc<dyn Transformer> {
        let t = match transformer.typ {
            transformers::TransformerTypes::NonInteractiveTransformer => {
                NonInteractiveTransformer::new(transformer.name.clone())
            }
        };

        Rc::new(t)
    }

    pub fn load_jobs(&mut self, jobs: &jobs::Jobs) -> Vec<Job> {
        jobs.iter().map(|j| self.load_job(j)).collect()
    }

    pub fn load_job(&mut self, job: &jobs::Job) -> Job {
        let transformer = self
            .transformers
            .get(&job.transformer)
            .expect("Transformer not found")
            .clone();

        let dependencies = self.load_dependencies(&job.dependencies);
        let targets = self.load_targets(&job.targets);

        Job::new(dependencies, targets, transformer)
    }

    pub fn load_targets(&mut self, targets: &jobs::Targets) -> Targets {
        targets
            .iter()
            .map(|t| Target(self.load_file_specifer(t)))
            .collect()
    }

    pub fn load_dependencies(&mut self, dependencies: &jobs::Dependencies) -> Dependencies {
        dependencies
            .iter()
            .map(|d| Dependency(self.load_file_specifer(d)))
            .collect()
    }

    pub fn load_file_specifer(
        &mut self,
        file_specifier: &jobs::FileSpecifier,
    ) -> Box<dyn FileSpecifier> {
        use super::jobs::FileSpecifierType;
        match file_specifier {
            jobs::FileSpecifier(FileSpecifierType::Regex, s) => {
                Box::new(SingleFileSpecifier::new(PathBuf::from(s)))
            }
            jobs::FileSpecifier(FileSpecifierType::SingleFile, s) => Box::new(
                RegexFileSpecifier::new(Regex::new(s).expect("invalid regex")),
            ),
        }
    }
}
