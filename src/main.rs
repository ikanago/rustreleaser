use config::Project;
use context::Context;
use pipeline::Pipeline;

mod action;
mod config;
mod context;
mod pipeline;

#[cfg(test)]
mod tests;

fn main() {
    let mut project = Project::default();
    project.dist = "tmp".to_string();
    let mut context = Context { project };
    let pipeline = Pipeline::Build;
    if let Err(err) = pipeline.run(&mut context) {
        eprintln!("{}", err);
    }
}
