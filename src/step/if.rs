#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct StepIf {
    test: String,
    if_true: Vec<AnyStep>,
    if_false: Vec<AnyStep>,
}
