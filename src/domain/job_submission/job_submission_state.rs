pub mod dead_state;
pub mod preparing_state;
pub trait JobSubmissionState {
    fn run(&self);
}
