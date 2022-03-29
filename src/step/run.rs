use std::{collections::HashMap, path::Path};

use crate::exec::Exec;

use super::{Operation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Source {
    Url(String),
    File(Box<Path>),
    Text(String),
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub struct Run {
    /// the interperter this script is for
    #[serde(default)]
    pub args: Vec<String>,

    #[serde(default)]
    pub ignore_environment: bool,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub env: HashMap<String, Option<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cwd: Option<String>,
    
    pub src: Source,
    pub interperter: Option<Interpreter>,

}



pub struct ExecuateSource {
    /// the interperter this script is for
    pub interpreter: Interpreter,

    pub args: Vec<String>,

    /// the command to run
    pub command: String,

    // what path to use
    pub cwd: Option<String>,
}

impl Operation for ExecuateSource {
    fn steps(&self) ->Vec<Exec> {
        let exec = self.interpreter.as_exec().arg("-c").arg(&self.command);
        vec![exec]
    }
}


/// Which interperter to use for executing a script
/// https://en.wikipedia.org/wiki/Interpreter_directive
#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub enum Interpreter {
    ///Execute the file using the Bourne shell, or a compatible shell, assumed to be in the /bin directory
    BourneShell,

    /// Execute the file using the Bash shell
    Bash,

    ///#!/usr/bin/pwsh – Execute the file using PowerShell
    PowerShell,

    ///using the env program search path to find it
    Env(String),
}

impl Interpreter {
    fn as_exec(&self) -> Exec {
        match self {
            Interpreter::BourneShell => Exec::new("/bin/sh"),
            Interpreter::Bash => Exec::new("/bin/bash"),
            Interpreter::PowerShell => Exec::new("/bin/pwsh"),
            Interpreter::Env(x) => {
                Exec::parse(&format!("/usr/bin/env {}" ,x)).expect("bad env argument")
            },
        }
    }
}
