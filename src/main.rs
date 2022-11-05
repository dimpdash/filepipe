use std::{path::{Path}, collections::HashMap};

use crate::controllers::job_submission_controller::JobSubissionControler;

pub mod domain;
pub mod controllers;
pub mod data_source;

fn main() {
    let inputs = vec![Path::new("./data.dat")];
    let program = "copy";


    JobSubissionControler::create_job(inputs, program, HashMap::new()).unwrap();
    JobSubissionControler::run_job();

}

