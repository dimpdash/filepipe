
pub mod preparing_state;
pub mod dead_state;
pub trait JobSubmissionState {
    fn run(&self);
}