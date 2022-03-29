use std::{path::Path, string};

use crate::exec::Exec;

use super::{Step};
use dockerfile::{Add, DockerfileBuilder, Run};
use serde::{Deserialize, Serialize, schemars::JsonSchema};

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub struct Block {
    pub steps: Vec<Exec>,
    pub cleanup: Vec<Exec>,
}

impl Step for Block {
    fn apply_to_dockerfile(&self, dockerfile: DockerfileBuilder) -> DockerfileBuilder {
        todo!()
    }
}
