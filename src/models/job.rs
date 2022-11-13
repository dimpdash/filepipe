use std::rc::Rc;

use derive_new::new;

use super::{file_specifier::Dependencies, file_specifier::Targets, transformer::Transformer};

#[derive(new)]
pub struct Job {
    dependencies: Dependencies,
    targets: Targets,
    transformer: Rc<dyn Transformer>,
}
