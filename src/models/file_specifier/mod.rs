use ambassador::delegatable_trait;

pub mod regex_file_specifier;
pub mod single_file_specifier;

#[delegatable_trait]
pub trait FileSpecifier {
    fn is_match(&self, s: String) -> bool;
}

use ambassador::Delegate;

#[derive(Delegate)]
#[delegate(FileSpecifier, automatic_where_clause = "false")]
pub struct Target(pub Box<dyn FileSpecifier>);

pub type Targets = Vec<Target>;

#[derive(Delegate)]
#[delegate(FileSpecifier, automatic_where_clause = "false")]
pub struct Dependency(pub Box<dyn FileSpecifier>);

pub type Dependencies = Vec<Dependency>;
