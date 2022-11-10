use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use filepipe::{
    app::App,
    controllers::job_submission_controller::JobSubissionController,
    data_source::{
        job_submission_repository::JobSubmissionRepository, program_repository::ProgramRepository,
    },
};

#[test]
fn create_job() {
    let inputs = vec![Path::new(
        "./tests/artefacts/generated/integration/job_controller_test/create_job/data.dat",
    )];

    let program = "copy";

    let program_config =
        PathBuf::from("./tests/artefacts/generated/integration/common/program_config");
    let program_repository = ProgramRepository::new(program_config);

    let default_job_location =
        PathBuf::from("./tests/artefacts/generated/integration/job_controller_test/create_job");
    let job_submission_repository = JobSubmissionRepository::new(default_job_location);

    let app = Rc::new(App::new(
        program_repository,
        Some(job_submission_repository),
    ));
    let js = JobSubissionController::new(app);

    js.create_job(inputs, program, HashMap::new(), None)
        .unwrap();
    JobSubissionController::run_job();
}
