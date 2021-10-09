use dockerfile::{Add, DockerfileBuilder};
use thiserror::Error;

use serde::{Deserialize, Serialize};

mod apt;
mod pip;
mod run;
mod comment;
mod download;

use crate::exec::Exec;

use self::apt::*;
use self::pip::*;
use self::run::*;
use self::comment::*;

#[enum_dispatch]
pub trait Operation {
    fn apply_to_dockerfile(&self, dockerfile: DockerfileBuilder) -> DockerfileBuilder{
        let steps = self.steps().into_iter();
        let shell_steps: Vec<_> = steps.map(Exec::to_shell).collect();
        let shell = shell_steps.join(" && ");
        dockerfile.push(dockerfile::Run::new(shell))
        //dockerfile.append(steps.map(|exec|Run::new(exec.to_shell())).collect())

    }
    fn steps(&self) -> Vec<Exec>;
    fn execute(&self) -> anyhow::Result<()> {
        //let mut out = Vec::new();
        for exec in self.steps(){
            let out = exec.execute()?;
            //out.push(out?);
        }
        Ok(())
        //Ok(out.join("\n"))
    }
}

#[enum_dispatch(Step)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyStep {
    AptRepoAdd(AptRepoAdd),
    AptInstall(AptInstall),
    PipInstall(PipInstall),
    Run(Run),
    Comment(Comment),
    // Env(StepEnvironment),
    // AptAddRepo(StepAptRepoAdd),
    // AptInstall(StepAptInstall),
    // PipInstall(StepPipInstall),
    // Script(StepScript),
    // Addon(StepAddon),
    // CreateUser(StepCreateUser),
}