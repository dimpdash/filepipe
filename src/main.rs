use std::{collections::HashMap, path::PathBuf, sync::Arc, vec};

use crate::domain::input::{self, input_list::InputList};
use crate::domain::{
    input::{single_file::SingleFileInput, Input},
    job_submission::JobSubmission,
    program::Program,
};
use serde::{Deserialize, Serialize};

use crate::{app::APP, domain::program::non_interactive_program::NonInteractiveProgram};

pub mod app;
pub mod controllers;
pub mod data_source;
pub mod domain;

#[derive(Serialize, Deserialize)]
struct B {
    id: u32,
}

struct C {
    name: String,
}

fn main() {
    let program = NonInteractiveProgram::new(PathBuf::from("./copy"));
    let mut app = APP.lock().unwrap();
    let program_repo = app.get_program_repo();
    program_repo.insert(Box::new(program));

    let mut job_submission_repo = app.get_job_submission_repo();

    let program = Arc::new(NonInteractiveProgram::new(PathBuf::from("copy")));
    let mut input_list = Box::new(InputList::new(vec![]));
    let input1: Box<dyn Input> = Box::new(SingleFileInput::new(PathBuf::from("./inputfile1.txt")));
    let input2: Box<dyn Input> = Box::new(SingleFileInput::new(PathBuf::from("./inputfile2.txt")));
    input_list.push(input1);
    input_list.push(input2);
    let inputs: Vec<Box<dyn Input>> = vec![
        Box::new(SingleFileInput::new(PathBuf::from("./inputfile.txt"))),
        input_list,
    ];

    let location = PathBuf::from("./job1");
    let parameters = HashMap::new();
    let job_submission = Arc::new(JobSubmission::new(program, inputs, location, parameters));
    job_submission_repo.insert(job_submission);

    let jobs = job_submission_repo.findAll().unwrap();
    let output = serde_yaml::to_string(&jobs).unwrap();

    println!("{}", output);

    // let f = fs::File::open(PathBuf::from("./test.yaml")).unwrap();

    // let r: A = serde_yaml::from_reader(&f).unwrap();

    // println!("{}", serde_yaml::to_string(&r).unwrap());
}
