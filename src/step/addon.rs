#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct Addon {
    #[serde(flatten)]
    id: ThingRefArg,
    with: HashMap<String, Value>,
}
