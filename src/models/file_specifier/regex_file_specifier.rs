use derive_new::new;
use regex::Regex;

use super::FileSpecifier;

#[derive(new)]
pub struct RegexFileSpecifier {
    regex: Regex,
}

impl FileSpecifier for RegexFileSpecifier {
    fn is_match(&self, s: String) -> bool {
        !todo!()
    }
}
