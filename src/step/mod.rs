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

    fn steps(&self) -> Vec<Exec>;

    fn entrypoint(&self) -> Vec<Exec> {
        Vec::new()
    }
    
    // docker options
    // like mounts, privlidged, capAdd, securityOpt

    fn post_install(&self) -> Vec<Exec> {
        Vec::new()
    }

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

impl dyn Operation {
    fn apply_to_dockerfile(&self, dockerfile: DockerfileBuilder) -> DockerfileBuilder{
        let steps = self.steps().into_iter();
        let shell_steps: Vec<String> = steps.map(|e| e.to_shell()).collect();
        let shell = shell_steps.join(" && ");
        dockerfile.push(dockerfile::Run::new(shell))
        //dockerfile.append(steps.map(|exec|Run::new(exec.to_shell())).collect())

    }
}

#[enum_dispatch(Step)]
#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyOperation {
    AptRepoAdd(AptRepoAdd),
    AptInstall(AptInstall),
    PipInstall(PipInstall),
    Run(Run),
    Comment(Comment),
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub struct Step{
    name: String,
    #[serde (flatten)] 
    operation: AnyOperation,
}