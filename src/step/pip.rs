use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::exec::Exec;
use crate::Operation;

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq, Default)]
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
    arguments: HashMap<String, Option<String>>,
}

impl Operation for PipInstall {
    fn steps(&self) -> Vec<Exec> {
        let mut args = Exec::new("python").args(["-m", "pip"]);

        if self.upgrade {
            args = args.arg("--upgrade");
        }

        if !self.keep_cache {
            args = args.arg("--no-cache-dir");
        }

        args = args.arg("install");

        for extra_index_url in &self.extra_index_urls {
            args = args.arg("--extra_index_url");
            args = args.arg(extra_index_url);
        }

        for find_link in &self.find_links {
            args = args.arg("--find-link");
            args = args.arg(find_link);
        }

        for (arg, value) in &self.arguments {
            args = args.arg(arg);
            assert!(arg.starts_with("--"), "Not a valid argument");
            if let Some(value) = value {
                args = args.arg(value);
            }
        }

        for requirement in &self.requirements {
            args = args.arg(requirement);
        }

        vec![args]
    }
}
