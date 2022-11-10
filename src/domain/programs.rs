use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use mockall::lazy_static;

use crate::domain::program::Program;
#[derive(Serialize, Deserialize)]
pub struct Programs {
    programs: HashMap<String, Arc<dyn Program>>,
}

impl Programs {
    pub fn new() -> Self {
        Programs {
            programs: HashMap::new(),
        }
    }
    pub fn get_program(&self, id: &str) -> Option<Arc<dyn Program>> {
        Some(self.programs.get(id)?.clone())
    }

    pub fn add_program(&mut self, program: Box<dyn Program>) {
        let _ = self.programs.insert(program.get_name(), Arc::from(program));
    }
}
