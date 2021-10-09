use serde::{Deserialize, Serialize};

use crate::exec::Exec;
use crate::step::Step;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AptInstall {
    update: bool,
    packages: Vec<String>,
    keep_cache: bool,
}

impl Step for AptInstall {
    fn steps(&self) -> Vec<Exec> {
        let mut cmds = Vec::new();
        if self.update {
            cmds.push(Exec::new("apt-get").arg("update"))
        }
        let meat = Exec::new("apt-get")
            .args(["-y", "install", "--no-install-recommends"])
            .env("DEBIAN_FRONTEND", "noninteractive");
        cmds.push(meat);
        if !self.keep_cache {
            cmds.push(Exec::new("rm").args(["-rf", "/var/lib/apt/lists/*"]));
            cmds.push(Exec::new("apt-get").arg("clean"));
        }
        cmds
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AptRepoAdd {
    name: String,
    sources_list: String,
    gpg_key: Option<String>,
}