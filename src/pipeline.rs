use anyhow::Result;

use crate::{action::{Action, build::Build}, context::Context};

pub enum Pipeline {
    Build,
}

impl Pipeline {
    pub fn run(&self, context: &mut Context) -> Result<()> {
        let actions = match self {
            Pipeline::Build => Self::create_build_pipeline(),
        };

        for action in actions {
            action.run(context)?;
        }

        Ok(())
    }

    fn create_build_pipeline() -> Vec<Box<dyn Action>> {
        vec![Box::new(Build)]
    }
}