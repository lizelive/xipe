use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub struct Exec {
    pub program: String,

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
}

impl Default for Exec {
    fn default() -> Self {
        Self {
            program: "true".to_string(),
            args: Vec::new(),
            ignore_environment: false,
            env: HashMap::new(),
            cwd: None,
        }
    }
}

use std::process::{Command, Output};
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("command is not valid format")]
    InvalidCommand,

    #[error("failed to start with io error {0}")]
    FailedToRun(#[from] std::io::Error),

    #[error("run failed")]
    RunFailed(Output),

    #[error("ran sucessfully but coulden't parse the utf-8 output")]
    ResultDecode(#[from] FromUtf8Error),
}

impl From<Output> for ExecuteError {
    fn from(o: Output) -> Self {
        assert!(o.status.success(), "this is sucess");
        Self::RunFailed(o)
    }
}

impl<T> From<Option<T>> for ExecuteError {
    fn from(o: Option<T>) -> Self {
        assert!(o.is_none(), "Some isn't an error");
        Self::InvalidCommand
    }
}

impl Exec {
    pub fn env(mut self, name: impl ToString, value: impl ToString) -> Self {
        self.env.insert(name.to_string(), Some(value.to_string()));
        self
    }

    pub fn arg(mut self, arg: impl ToString) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn args<S: ToString>(self, args: impl IntoIterator<Item = S>) -> Self {
        let mut this = self;
        for arg in args {
            this = this.arg(arg);
        }
        this
    }

    pub fn with_env(
        program: String,
        args: Vec<String>,
        ignore_environment: bool,
        env: HashMap<String, Option<String>>,
        cwd: Option<String>,
    ) -> Self {
        Self {
            program,
            args,
            ignore_environment,
            env,
            cwd,
        }
    }

    pub fn to_shell(&self) -> String {
        let mut program = String::new();

        if self.cwd.is_some() || self.ignore_environment || !self.env.is_empty() {
            program += "env";

            if self.ignore_environment {
                program += " --ignore-environment";
            }

            if let Some(cwd) = &self.cwd {
                program+=&format!(" --chdir={}", shlex::quote(cwd));
            }

            for (k, v) in &self.env {
                if let Some(v) = v {
                    let v = shlex::quote(v);
                    program+=&format!(" {}={}", k, v)
                } else {
                    program+= &format!(" --unset={}", k);
                }
            }
        }

        program += &self.program;

        for arg in &self.args {
            program += " ";
            program += &shlex::quote(arg);
        }

        program
    }

    pub fn parse(in_str: &str) -> Option<Self> {
        let args = shlex::split(in_str)?;
        let (program, args) = args.split_first()?;
        Some(Self::new(program).args(args))
    }

    pub fn execute(&self) -> Result<String, ExecuteError> {
        
        let mut command = Command::new(&self.program);
        command.args(&self.args);
        let out = command.output()?;
        if out.status.success() {
            let str = String::from_utf8(out.stdout)?;
            Ok(str)
        } else {
            let str = String::from_utf8(out.stderr)?;
            Ok(str)
            //Err(out.into())
        }
    }

    pub fn new(program: impl ToString) -> Exec {
        Self {
            program: program.to_string(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_exec() {
        let cmd = Exec::new("true").arg("hello world");
    }
}
