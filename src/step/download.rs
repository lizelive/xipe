use std::{path::Path, string};

use crate::exec::Exec;

use crate::Step;
use dockerfile::{Add, DockerfileBuilder, Run};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Download {
    pub url: String,
    pub dest: String,
}

impl Step for Download {
    fn steps(&self) -> Vec<Exec> {
        todo!()
    }
}
