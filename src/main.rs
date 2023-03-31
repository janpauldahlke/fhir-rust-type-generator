#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct FhirRessource {
    ressource_type: String,
}

fn read_and_write_types() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./fhir-json-raw/definitions/fhir.schema.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let defintions: Vec<FhirRessource> = serde_json::from_str(&data)?;

    for def in defintions {
        let rust_type = generate_rust_fhir_type(&def);
        let mut file = File::create(format!("./fhir-types/{}.rs", def.ressource_type))?;
        file.write_all(rust_type.as_bytes())?;
    }
    Ok(())
}

//this should generate our rust type files
fn generate_rust_fhir_type(definition: &FhirRessource) -> String {
    let mut fhir_type = String::new();
    fhir_type.push_str("use serde::{Deserialize, Serialize};\n");
    fhir_type.push_str("#[derive(Serialize, Deserialize, Debug)]\n");
    fhir_type.push_str(&format!("pub struct {} {{", definition.ressource_type));
    fhir_type.push_str("}\n");
    fhir_type
}

fn main() {
    //read_from_single_src()
    //read_from_src_dir();
    read_and_write_types().unwrap();
}

fn read_from_single_src() {
    let mut file = File::open("./fhir-json-raw/fhir_5.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("With text:\n{}", contents);
}

fn read_from_src_dir() {
    let path = fs::read_dir("./fhir-json-raw/examples-json/").unwrap();

    for p in path {
        match p {
            Ok(p) => {
                let mut file = File::open(p.path()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("With text:\n{}", contents);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
