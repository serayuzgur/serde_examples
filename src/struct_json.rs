//! # Struct to Json String and reverse.
//! This sample aims to show the usage of serde for the following scenarios.
//!
//! * Serialize struct to json string.
//! * Deserialize json string to struct.
extern crate serde_json;
/// Shows the basic usage of the scenario.
pub fn run() {
    let sample = Sample {
        name: "Joe".to_string(),
        age: 30,
    };
    println!("* struct_json");

    // First serialize struct to json string.
    let serialized = serde_json::to_string(&sample).unwrap();
    println!("\tSerialized : {}", &serialized);

    // Than deserialize json string to struct.
    let deserialized: Sample = serde_json::from_str(&serialized).unwrap();
    println!("\tDeserialized: {:?}", &deserialized);
}

//These are for auto ser,de founction generation.
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
/// A sample struct.
struct Sample {
    name: String,
    age: u8,
}

