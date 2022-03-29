use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    marker::PhantomData,
    str::FromStr,
};

use chrono::{DateTime, Utc};
use semver::{VersionReq, Version};
use serde_with::{serde_as, DisplayFromStr};

type Statement = Box<AnyStatement>;
type Expr = Box<AnyExpr>;


#[derive(serde::Serialize, serde::Deserialize, Debug, schemars::JsonSchema, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BinaryOperator {
    Part,

    // predicates
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,

    // set stuff
    In,
    Is,
    El,

    // math
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    Shl,
    Shr,
}

type Number = isize;
type Sequence = Vec<Value>;
type Mapping = BTreeMap<String, Value>;

// #[serde_as]
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct Version {
//     #[serde_as(as = "DisplayFromStr")]
//     version: semver::Version,
// }

//#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Debug, schemars::JsonSchema, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]

pub enum Value {
    /// Represents a YAML null value.
    Null,
    
    
    Error(String, Box<Value>),

    /// Represents a YAML boolean.
    Bool(bool),
    /// Represents a YAML numerical value, whether integer or floating point.
    Number(Number),

    // DateTime(DateTime<Utc>),

    // Version {
    //     #[serde_as(as = "DisplayFromStr")]
    //     version: Version,
    // },

    // VersionReq {
    //     #[serde_as(as = "DisplayFromStr")]
    //     req: VersionReq,
    // },

    /// Represents a YAML string.
    String(String),
    /// Represents a YAML sequence in which the elements are
    /// `serde_yaml::Value`.
    Sequence(Sequence),
    /// Represents a YAML mapping in which the keys and values are both
    /// `serde_yaml::Value`.
    Mapping(Mapping),
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

type Integer = isize;
type Symbol = String;

enum AnyExpr {
    Value(Value),
    Get(Symbol),
    BinaryOperator(Expr, BinaryOperator, Expr),
    Call(Symbol, Expr),
}

enum AnyStatement {
    Set(Symbol, Expr),
    If(Expr, Statement, Statement),
    CleanUp(Statement, Statement),
}
