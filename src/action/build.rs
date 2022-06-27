use std::process::Command;

use super::Action;
use crate::context::Context;
use anyhow::{anyhow, Result};

pub struct Build;

impl Action for Build {
    fn run(&self, context: &mut Context) -> Result<()> {
        Command::new("cargo")
            .args(["build", "--target-dir", &context.project.dist])
            .output()
            .map_err(|err| anyhow!("Failed to execute process: {:?}", err))?;
        Ok(())
    }
}
