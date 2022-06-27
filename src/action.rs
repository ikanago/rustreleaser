use crate::context::Context;

pub mod build;
pub mod cargo_config;

pub trait Action {
    fn run(&self, context: &mut Context) -> anyhow::Result<()>;
}
