use std::{
    path::{PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    app::{APP},
    domain::program::{non_interactive_program::NonInteractiveProgram},
};

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
    let programs = app.get_programs();
    programs.add_program(Box::new(program));

    let output = serde_yaml::to_string(&app.get_programs()).unwrap();

    println!("{}", output);

    // let f = fs::File::open(PathBuf::from("./test.yaml")).unwrap();

    // let r: A = serde_yaml::from_reader(&f).unwrap();

    // println!("{}", serde_yaml::to_string(&r).unwrap());
}
