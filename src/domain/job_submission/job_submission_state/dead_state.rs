use super::JobSubmissionState;



pub struct DeadState {
    
}

impl JobSubmissionState for DeadState {
    fn run(&self) {
        todo!()
    }
}