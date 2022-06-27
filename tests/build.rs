use rustreleaser::config::Project;
use rustreleaser::context::Context;
use rustreleaser::pipeline::Pipeline;
use std::env;

#[test]
fn build_simple_project() {
    env::set_current_dir("./tests/fixtures/simple").unwrap();

    let mut project = Project::default();
    project.dist = "tmp".to_string();
    let mut context = Context { project };
    let pipeline = Pipeline::Build;
}
