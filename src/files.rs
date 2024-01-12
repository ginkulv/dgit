use std::{fs, io::Error};

use serde_json::{from_reader, to_string};
use crate::dbconnector::{Entity, Credentials};

pub fn read_credentials(dir_path: &str) -> Result<Credentials, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/credentials");
    let file = fs::File::open(&path)?;
    let credentials: Credentials = from_reader(&file).unwrap();
    Ok(credentials)
}

pub fn store_credentials(dir_path: &str, credentials: Credentials) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/credentials");
    let json = to_string(&credentials).unwrap();
    std::fs::write(&path, &json)?;
    Ok(())
}

pub fn read_staged_entities(dir_path: &str) -> Result<Vec<Entity>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/stage");
    let file = fs::File::open(&path)?;
    let entities: Vec<Entity> = from_reader(&file).unwrap();
    Ok(entities)
}

pub fn store_staged(dir_path: &str, entities_to_stage: Vec<Entity>) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/stage");
    let json = to_string(&entities_to_stage).unwrap();
    fs::write(&path, &json)?;
    Ok(())
}
