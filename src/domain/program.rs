use mockall::automock;

pub mod non_interactive_program;

#[automock]
pub trait Program {
    fn get_name(&self) -> String;
}