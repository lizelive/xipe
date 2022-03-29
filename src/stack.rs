use crate::{Argument, PackageInfo, AnyOperation, Requirements, ThingRef};

#[derive(serde::Serialize, schemars::JsonSchema, serde::Deserialize, Debug, PartialEq)]
pub struct Stack {
    #[serde(default)]
    args: Vec<Argument>,

    #[serde(flatten)]
    meta: PackageInfo,

    steps: Vec<AnyOperation>,

    requirements: Requirements,

    /// what mixins this stack depends on
    #[serde(default)]
    mixins: Vec<ThingRef>,
}

