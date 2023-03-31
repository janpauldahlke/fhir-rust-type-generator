use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct BasicType {
    resource_type: String,
}
fn main() {
    println!("Hello, world!");
    read_and_write_types();
}

fn read_and_write_types() {
    let mut file =
        File::open("./fhir-json-raw/definitions/fhir.schema.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let definitions: HashMap<String, Value> = serde_json::from_str(&contents).unwrap();
    let mut counter = 1;
    for key in definitions {
        if key.0 == "discriminator".to_owned() {
            println!("found discriminator");
            let resource_type = key.0;
            let definition = key.1;

            for (field, value) in definition.as_object().unwrap() {
                if field == "mapping" {
                    println!("found mapping");
                    for (field, value) in value.as_object().unwrap() {
                        let types = generate_rust_fhir_type(field, value);
                        println!("{}", types);

                        let mut file = File::create(format!("./fhir-types/{}.rs", resource_type))
                            .expect("msg");
                        file.write_all(types.as_bytes());
                    }
                } else {
                    println!("unable to create type for {}", field);
                }
            }
        } else {
            println!("irrelevant key: {}", key.0);
        }
        counter += 1;
    }
}

fn generate_rust_fhir_type(resource_type: &str, definition: &Value) -> String {
    let mut fhir_type = String::new();
    fhir_type.push_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n");
    fhir_type.push_str(&format!("pub struct {} {{", { resource_type }));
    fhir_type.push_str("\n}\n");
    fhir_type
}
