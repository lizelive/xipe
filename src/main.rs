use std::fs::File;
use std::io::Write;
use std::{borrow::Cow, collections::HashMap};

mod step;

pub use crate::step::{AnyOperation, Operation};

use serde::{Deserialize, Serialize};

mod ast;
mod exec;
mod stack;

use crate::stack::*;

use crate::ast::Value;

#[macro_use]
extern crate enum_dispatch;

use anyhow::anyhow;

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
enum DockerCommand {
    Run { command: String },
    Label(String, String),
    Add(String, String),
    Arg(String, String),
    From(String),
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct ThingIdentifier {
    namespace: String,
    name: String,
    label: String,
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum Requirements {
    Compare(String, Comparison, Value),
    None { none: Vec<Requirements> },
    All { all: Vec<Requirements> },
    Any { any: Vec<Requirements> },
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
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
                Value::String(other) => {
                    let value_str = coerce_to_string(value);
                    Ok(other.contains(&value_str))
                }
                Value::Sequence(seq) => Ok(seq.contains(value)),
                //Value::Mapping(map) => Ok(map.contains_key(value)),
                _ => Err(anyhow! {"can't be in {:?}", other}),
            },
            Comparison::Is => match (value, other) {
                (Value::Number(_), Value::String(_)) => todo!(),
                (Value::String(_), Value::Number(_)) => todo!(),
                (Value::String(value), Value::String(other)) => {
                    let value = semver::Version::parse(value)?;
                    let other = semver::VersionReq::parse(other)?;
                    Ok(other.matches(&value))
                }
                _ => Err(anyhow! {"{:?} can't be {:?}", value, other}),
            },
        }
    }
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct ThingRef {
    #[serde(flatten)]
    id: ThingIdentifier,
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct ThingRefArg {
    #[serde(flatten)]
    id: ThingRef,
    args: Option<HashMap<String, Value>>,
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
pub enum Licence {
    Unlicensed,
    Spdx(String),
    Url(String),
    Custom(String),
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct PackageInfo {
    #[serde(flatten)]
    id: ThingIdentifier,

    license: Option<Licence>,
    description: Option<String>,

    author: Option<String>,
    maintainer: Option<String>,

    #[serde(default)]
    labels: HashMap<String, Value>,
}

// #[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
// enum DigestOrTag {
//     Tag(String),
//     Digest(String),
// }

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct DockerImageRef {
    registry: Option<String>,
    name: String,
    digest: Option<String>,
    tag: Option<String>,
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum Base {
    Docker(DockerImageRef),
    Image(ThingRef),
}

#[derive(Debug, PartialEq, Serialize, schemars::JsonSchema, Deserialize)]
struct Image {
    #[serde(flatten)]
    id: ThingRefArg,
    /// dockerfile that is the base of this image
    base: Base,

    /// The stack to use
    stack: ThingRefArg,

    /// what mixins to bake for this stack
    #[serde(default)]
    mixins: Vec<ThingRef>,
}

#[derive(Debug, PartialEq, Serialize, schemars::JsonSchema, Deserialize)]
struct Repo {
    images: Vec<Image>,
    stacks: Vec<Stack>,
}

fn main() -> anyhow::Result<()> {
    let schema = schemars::schema_for!(Repo);
    let schema_string = serde_json::to_string_pretty(&schema).unwrap();
    //println!("{}", schema_string);
    let mut output = File::create("schema.json")?;
    output.write_all(schema_string.as_bytes())?;
    //write!(output, "{}", schema_string);

    let cool = include_str!("../test/images/azureml-interactive-host.json");
    let image: Image = serde_yaml::from_str(cool)?;
    println!("{:?}", image);
    Ok(())
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
