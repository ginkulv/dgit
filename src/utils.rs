use std::fs;
use std::path::Path;
use std::collections::BTreeMap;
use std::io::Error;
use serde_json::{from_reader, to_string};
use crate::dbconnector::Entity;

pub fn repo_exists(dir_path: &str) -> bool {
    let path = format!("{}{}", dir_path, "/.dgit");
    Path::new(&path).exists()
}

pub fn read_credentials(dir_path: &str) -> Result<BTreeMap<String, String>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/.credentials");
    let file = fs::File::open(&path)?;
    let credentials: BTreeMap<String, String> = from_reader(&file).unwrap();
    Ok(credentials)
}

pub fn store_credentials(dir_path: &str, credentials: &BTreeMap<&str, &str>) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/.credentials");
    let yaml = to_string(&credentials).unwrap();
    std::fs::write(&path, &yaml)?;
    Ok(())
}

pub fn store_added_entities(dir_path: &str, changed: &BTreeMap<String, Vec<String>>) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/add");
    let yaml = to_string(changed).unwrap(); 
    fs::write(&path, &yaml)?;
    Ok(())
}

pub fn read_added_entities(dir_path: &str) -> Result<Vec<Entity>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/add");
    let file = fs::File::open(&path)?;
    let changes: BTreeMap<String, Vec<String>> = from_reader(&file).unwrap();
    let tables: Vec<String> = changes.get("tables").unwrap().to_vec();
    let mut entities: Vec<Entity> = Vec::new();
    for table in tables {
        let names: Vec<&str> = table.split(".").collect();
        let entity: Entity = Entity::new(names.get(0).unwrap().to_string(), names.get(1).unwrap().to_string());
        entities.push(entity);
    }
    Ok(entities)
}
pub fn strip_trailing_newline(input: &str) -> String {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

pub fn parse_argument(argument: &str) -> Result<Entity, String> {
    let result: Vec<&str> = argument.split(".").collect();
    if result.len() == 1 {
        return Err(format!("Invalid argument provided: {}", &argument))
    }
    let domain = result[0];
    let name = result[1];
    Ok(Entity::new(String::from(domain), String::from(name)))
}
