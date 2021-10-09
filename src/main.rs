use std::{borrow::Cow, collections::HashMap};

mod step;

pub use crate::step::{Operation, AnyStep};



use serde::{Deserialize, Serialize};
use serde_yaml::Value;

mod ast;

mod exec;

#[macro_use]
extern crate enum_dispatch;

use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum DockerCommand {
    Run { command: String },
    Label(String, String),
    Add(String, String),
    Arg(String, String),
    From(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Argument {
    name: String,
    default: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ThingIdentifier {
    namespace: String,
    name: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum Requirements {
    Compare(String, Comparison, Value),
    None { none: Vec<Requirements> },
    All { all: Vec<Requirements> },
    Any { any: Vec<Requirements> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Comparison {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    In,
    Is,
}

fn coerce_to_string(value: &Value) -> String {
    if let Value::String(str) = value {
        str.clone()
    } else {
        serde_yaml::to_string(value).expect("value should never fails to serlize")
    }
}

impl Comparison {
    fn compare(&self, value: &Value, other: &Value) -> anyhow::Result<bool> {
        let compare = value.partial_cmp(other);
        match self {
            Comparison::Eq => Ok(value.eq(other)),
            Comparison::Ne => Ok(value.ne(other)),
            Comparison::Gt => Ok(value.gt(other)),
            Comparison::Lt => Ok(value.lt(other)),
            Comparison::Ge => Ok(value.ge(other)),
            Comparison::Le => Ok(value.le(other)),

            Comparison::In => match other {
                _ => Err(anyhow!{"can't be in {:?}", other}),
                Value::String(other) => {
                    let value_str = coerce_to_string(value);
                    Ok(other.contains(&value_str))
                }
                Value::Sequence(seq) => Ok(seq.contains(value)),
                Value::Mapping(map) => Ok(map.contains_key(value)),
            },
            Comparison::Is => {
                match (value, other) {
                    (Value::Number(_), Value::String(_)) => todo!(),
                    (Value::String(_), Value::Number(_)) => todo!(),
                    (Value::String(value), Value::String(other)) => {
                        let value = semver::Version::parse(value)?;
                        let other = semver::VersionReq::parse(other)?;
                        Ok(true)
                    },
                    _ => Err(anyhow!{"{:?} can't be {:?}", value, other})
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ThingRef {
    #[serde(flatten)]
    id: ThingIdentifier,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ThingRefArg {
    id: ThingRef,
    args: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PackageInfo {
    #[serde(flatten)]
    id: ThingIdentifier,

    license: Option<String>,
    description: Option<String>,
    maintainer: Option<String>,

    #[serde(default)]
    labels: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    username: String,
    gid: i32,
    uid: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct StepCreateUser {
    username: String,
    gid: i32,
    uid: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct StepAddon {
    #[serde(flatten)]
    id: ThingRefArg,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct StepEnvironment {
    env: Vec<(String, String)>,
    save: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum Base {
    Any,
    Docker(String),
    Image(ThingRef),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Stack {
    #[serde(flatten)]
    meta: PackageInfo,

    steps: Vec<AnyStep>,

    requirements: Requirements,

    #[serde(default)]
    args: Vec<Argument>,

    // what mixins are required for this stack
    #[serde(default)]
    mixins: Vec<ThingRef>,
}

impl Stack {
    fn can_go_on(&self, _base: &Base) -> bool {
        todo!()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BakeImage {
    // docker image that is the base of this image
    base: String,

    // The stack to use
    stack: ThingRefArg,

    // what mixins to bake for this stack
    #[serde(default)]
    mixins: Vec<ThingRef>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Repo {
    stacks: Vec<BakeImage>,
    addons: Vec<Stack>,
}

fn main() {
    // let data = FullStack {
    //     id: ThingIdentifier {
    //         name: "hello".to_string(),
    //         label: "world".to_string(),
    //     },
    //     args: Vec::default(),
    //     mixins: vec![],
    //     meta: PackageInfo { license: None, description: None, maintainer: None, labels: HashMap::new() },
    //     base: ThingRef{id: ThingIdentifier::new("hello".to_string(), "world".to_string())},
    //     steps: vec![
    //     ],
    // };

    // let nice = serde_json::to_string_pretty(&data)?;
    // let _stack: FullStack = serde_json::from_str(&nice)?;
    // println!("{}", nice);

    //println!("{} {:#?}", nice, stack);
}
