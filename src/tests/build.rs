use crate::config::Project;
use crate::context::Context;
use crate::pipeline::Pipeline;
use std::env;

#[test]
fn build_simple_project() {
    env::set_current_dir("./tests/fixtures/simple").unwrap();

    let mut project = Project::default();
    project.dist = "tmp".to_string();
    let mut context = Context { project };
    let pipeline = Pipeline::Build;
    pipeline.run(&mut context).unwrap();
    assert_eq!("simple", &context.project.project);
    assert_eq!("0.1.0", &context.project.version);
}
