use std::{fs, collections::BTreeMap, io::Error};

use serde_json::{from_reader, to_string};

use crate::dbconnector::Entity;

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

pub fn read_staged(dir_path: &str) -> Result<Vec<Entity>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/add");
    let file = fs::File::open(&path)?;
    let tables: Vec<String> = from_reader(&file).unwrap();
    let mut entities: Vec<Entity> = Vec::new();
    for table in tables {
        let names: Vec<&str> = table.split(".").collect();
        let entity: Entity = Entity::new(
            names.get(0).unwrap().to_string(),
            names.get(1).unwrap().to_string()
        );
        entities.push(entity);
    }
    Ok(entities)
}

pub fn store_staged(dir_path: &str, changed: &Vec<String>) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/add");
    let yaml = to_string(changed).unwrap(); 
    fs::write(&path, &yaml)?;
    Ok(())
}

