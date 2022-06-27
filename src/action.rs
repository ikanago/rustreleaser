use crate::context::Context;

pub mod build;

pub trait Action {
    fn run(&self, context: &mut Context) -> anyhow::Result<()>;
}
