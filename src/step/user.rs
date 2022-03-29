#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct CreateUser {
    username: String,
    gid: i32,
    uid: i32,
}

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
struct User {
    username: String,
    gid: i32,
    uid: i32,
}

