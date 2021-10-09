use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub fn env(self, name: impl ToString, value: impl ToString) -> Self {
        self.env.insert(name.to_string(), Some(value.to_string()));
        self
    }

    pub fn arg(self, arg: impl ToString) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn args<S: ToString>(self, args: impl IntoIterator<Item = S>) -> Self {
        self.args
            .append(args.into_iter().map(|x| x.to_string()).collect());
        self
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
        let program = Vec::new();

        if self.cwd.is_some() || self.ignore_environment || !self.env.is_empty() {
            program.push("env");

            if self.ignore_environment {
                program.push("--ignore-environment");
            }

            if let Some(cwd) = &self.cwd {
                program.push(&format!("--chdir={}", shlex::quote(cwd)))
            }

            for (k, v) in &self.env {
                if let Some(v) = v {
                    let v = shlex::quote(v);
                    program.push(&format!("{}={}", k, v))
                } else {
                    program.push(&format!("--unset={}", k))
                }
            }
        }

        program.push(&self.program);

        // for arg in &self.args {
        //     program.push(arg)
        // }

        let args_quoted = self.args.iter().map(|arg| shlex::quote(arg).as_ref());

        let program = program.into_iter().chain(args_quoted);

        shlex::join(program)
    }

    pub fn parse(in_str: &str) -> Option<Self> {
        let args = shlex::split(in_str)?;
        let (program, args) = args.split_first()?;
        Some(Self::new(program).args(args))
    }

    pub fn execute(&self) -> Result<String, ExecuteError> {
        let command = Command::new(self.program).args(self.args);
        let out = command.output()?;
        if out.status.success() {
            let str = String::from_utf8(out.stdout)?;
            Ok(str)
        } else {
            Err(out)
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
