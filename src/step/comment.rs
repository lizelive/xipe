use std::{path::Path, string};

use crate::exec::Exec;

use super::{Step};
use dockerfile::{Add, DockerfileBuilder, Run};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Comment {
    pub comment: String,
}

impl Step for Comment {
    fn steps(&self) ->Vec<Exec> {
        vec![
            Exec::new("echo").arg(self.comment)
        ]
    }
}