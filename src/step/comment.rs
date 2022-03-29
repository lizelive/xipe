use std::{path::Path, string};

use crate::exec::Exec;

use super::{Operation};
use dockerfile::{Add, DockerfileBuilder, Run};
use serde::{Deserialize, Serialize};

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub struct Comment {
    pub comment: String,
}

impl Operation for Comment {
    fn steps(&self) ->Vec<Exec> {
        vec![
            Exec::new("echo").arg(&self.comment)
        ]
    }
}