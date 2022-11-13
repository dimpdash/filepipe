use std::path::PathBuf;

use derive_new::new;

use super::FileSpecifier;

#[derive(new)]
pub struct SingleFileSpecifier {
    path: PathBuf,
}

impl FileSpecifier for SingleFileSpecifier {
    fn is_match(&self, s: String) -> bool {
        !todo!()
    }
}
