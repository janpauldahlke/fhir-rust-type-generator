#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

struct Definition {}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FhirRessource {}

impl fmt::Display for FhirRessource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn read_and_write_types() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./fhir-json-raw/definitions/fhir.schema.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let definitions: HashMap<String, FhirRessource> = serde_json::from_str(&data).unwrap();

    for def in definitions {
        println!("Generating type for {}", def);
        let rust_type = generate_rust_fhir_type(&def);
        let mut file = File::create(format!("./fhir-types/{}.rs", def.Can))?;
        file.write_all(rust_type.as_bytes())?;
    }
    Ok(())
}

//this should generate our rust type files
fn generate_rust_fhir_type(definition: &FhirRessource) -> String {
    let mut fhir_type = String::new();
    fhir_type.push_str("use serde::{Deserialize, Serialize};\n");
    fhir_type.push_str("#[derive(Serialize, Deserialize, Debug)]\n");
    fhir_type.push_str(&format!("pub struct {} {{", definition.resource_type));
    fhir_type.push_str("}\n");
    fhir_type
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    read_and_write_types()?;
    Ok(())
}
