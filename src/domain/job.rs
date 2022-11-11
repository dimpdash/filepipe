use std::rc::Rc;

use super::{dependency::Dependencies, target::Targets, transformer::Transformer};

pub struct Job {
    dependencies: Dependencies,
    targets: Targets,
    transformer: Rc<dyn Transformer>,
}
