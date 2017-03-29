//! # Struct to Json String and reverse with Attributes.
//! This sample aims to show the usage of serde for the following scenarios.
//!
//! * #[serde(rename_all = "camelCase")] Attribute for changing the naming.
//! * #[serde(default = "default_name")] Attribute to give default values for the missing json fields.
//! * #[serde(skip_serializing)] for skipping desired fields.

extern crate serde_json;
/// Shows the basic usage of the scenario.
pub fn run() {

    let json_string = r#" {
        "name":"Joe",
        "age":40,
        "snakeCase":"snakeCase",
        "password":"123",
        "children":[
            {
                "name":"Jonn",
                "age":15,
                "snakeCase":"snakeCase",
                "password":"123"
            }
        ]}"#;
    println!("\n* struct_json_attributes");

    // First deserialize json string to struct.
    let deserialized: Sample = serde_json::from_str(&json_string).unwrap();
    println!("\tDeserialized: {:?}", &deserialized);

    // Than serialize struct to json string.
    let serialized = serde_json::to_string(&deserialized).unwrap();
    println!("\tSerialized : {}", &serialized);

}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
struct Sample {
    name: String,
    age: u8,
    //This field will be camelCased
    snake_case: String,

    #[serde(default = "default_children")]
    children: Vec<Sample>,

    //Do not serialize password
    #[serde(skip_serializing)]
    password: String

}

///Sets [] if field is missing at the json string.
fn default_children() -> Vec<Sample> {
    vec![]
}

