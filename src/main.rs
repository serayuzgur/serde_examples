//! # Serde Examples
//! This project aims to show the usage of serde project with a simple project structure and simple examples. 
//! Common usages will be covered for helping everyday scenarios. 
//!
//! It is runnable. You can see each module is covering one spesific usage.

#[doc(hidden)]
#[macro_use]
extern crate serde_derive; // we have to define it here because macros must be at root 

pub mod struct_json;
pub mod struct_json_attributes;

fn main() {
    println!("---Examples---");
    struct_json::run();
    struct_json_attributes::run();
}

