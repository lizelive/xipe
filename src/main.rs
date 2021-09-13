use serde::{Deserialize, Serialize, de::value::BoolDeserializer};
use serde_yaml::{Result, Value};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum DockerCommand {
    Run{ command: String },
    Label(String, String),
    Add(String, String),
    Arg(String, String),
    From(String)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Argument{
    name : String,
    default: 
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum EStep {
    Environment,
    Args(Vec<Argument>),
    Pip,
    Mixin,
    Script
}

trait Step {
    fn execute(&self, env: &Enviorment) -> Result<()>;
    fn from(&self, value: Value) -> Result<()>;
    fn get_docker_commands(&self, env: &Enviorment) -> Vec<DockerCommand>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StepPip {
    user: Option<String>,
    packages: Vec<String>, 
}

impl Step for StepPip {
    fn execute(&self, env: &Enviorment) -> Result<()> {
        todo!()
    }

    fn from(&self, value: Value) -> Result<()> {
        todo!()
    }

    fn get_docker_commands(&self, env: &Enviorment) -> Vec<DockerCommand> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Enviorment{
    name: String,
    label: String,
    steps: Value
}

fn parse_step(kind: &Value, value: &Value) -> Box<dyn Step>{
    let kind: String = serde_yaml::from_value(kind.clone()).expect("yeah this easy");
    let something = match kind.as_str() {
        "pip" => match value {
            Value::Mapping(map) => {
                let step: StepPip = serde_yaml::from_value(value.clone()).expect("fuck error handling");
                Box::new(step)
            },
            Value::Sequence(stuff) =>{
                let packages:Vec<String> = serde_yaml::from_value(value.clone()).expect("not possible"); 
                Box::new(StepPip{packages, user: None})
            },
            _ => panic!("thats wrong")
        }
        _ => panic!("not a valid step kind")
    };
    return something;
}

fn main() -> Result<()> {
    let data = r#"---
name: AzureML/Interactive-JupyterLab
label: '3.1'
features:
    jupyterlab: '3.1'
base:
    name: AzureML/Interactive-Basic
    label: ubuntu20.04-python3.8-cuda11.4-cudnn8-reln0.5
steps:
    pip:
    - jupyterlab~=3.1"#;
    let env:Enviorment = serde_yaml::from_str(data)?;
    let mut steps: Vec<Box<dyn Step>> = vec![];
    match env.steps {
        Value::Mapping(stepmap) => for (kind,body) in stepmap {
            steps.push(parse_step(&kind, &body));
        },
        Value::Sequence(stepvec) => for step in stepvec {
            let kind = step.get("kind").expect("need to know kind");
            steps.push(parse_step(kind, &step));
        },
        _ => panic!("Bad steps")
    };

    println!("i have {} steps", steps.len());
    Ok(())
}
