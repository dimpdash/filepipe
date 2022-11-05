use crate::controllers::job_submission_controller::JobSubissionControler;

pub mod domain;
pub mod controllers;

fn main() {
    println!("Hello, world!");

    let x = JobSubissionControler::run_job();

    println!("{x}");
}

