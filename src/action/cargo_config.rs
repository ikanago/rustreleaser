use std::{fs::File, io::Read};

use super::Action;
use crate::context::Context;
use anyhow::anyhow;
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
}

pub struct CargoConfig;

impl Action for CargoConfig {
    fn run(&self, context: &mut Context) -> anyhow::Result<()> {
        let mut cargo_toml =
            File::open("Cargo.toml").map_err(|err| anyhow!("Cargo.toml not found: {}", err))?;
        let mut content = String::new();
        cargo_toml
            .read_to_string(&mut content)
            .map_err(|err| anyhow!("Failed to load Cargo.toml: {}", err))?;

        let toml_content: CargoToml = toml::from_str(&content)
            .map_err(|err| anyhow!("Cargo.toml has invalid format: {}", err))?;
        context.project.project = toml_content.package.name;
        context.project.version = toml_content.package.version;
        Ok(())
    }
}
