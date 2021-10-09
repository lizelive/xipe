use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::exec::Exec;
use crate::Step;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct PipInstall {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    requirements: Vec<String>,

    #[serde(default)]
    upgrade: bool,
    
    #[serde(default)]
    keep_cache: bool,

    #[serde(default)]
    extra_index_urls: Vec<String>,

    #[serde(default)]
    find_links: Vec<String>,

    #[serde(default)]
    arguments: HashMap<String, serde_yaml::Value>,
}

impl Step for PipInstall {
    fn steps(&self) -> Vec<Exec> {
        let mut args = Exec::new("python").args(["-m", "pip"]);

        if self.upgrade {
            args.arg("--upgrade");
        }

        if !self.keep_cache {
            args.arg("--no-cache-dir");
        }

        args.arg("install");

        for extra_index_url in &self.extra_index_urls {
            args.arg("--extra_index_url");
            args.arg(extra_index_url);
        }

        for find_link in &self.find_links {
            args.arg("--find-link");
            args.arg(find_link);
        }

        for (arg, value) in &self.arguments {
            args.arg(arg);
            assert!(arg.starts_with("--"), "Not a valid argument");
            if !value.is_null() {
                args.arg(&value.to_string())
            }
        }

        for requirement in &self.requirements {
            args.arg(requirement);
        }

        vec![args]
    }
}
